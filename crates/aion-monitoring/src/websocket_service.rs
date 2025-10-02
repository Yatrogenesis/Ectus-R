//! WebSocket Service for Real-time Dashboard Updates
//!
//! This module provides WebSocket functionality for streaming real-time metrics,
//! alerts, and dashboard updates to connected clients.

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Path, Query},
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use crate::real_time_monitor::{RealTimeMonitor, DashboardUpdate, MetricUpdate, AlertEvent};

/// WebSocket service for real-time monitoring
pub struct WebSocketService {
    monitor: Arc<RealTimeMonitor>,
    connections: Arc<RwLock<HashMap<String, ClientConnection>>>,
}

/// Client connection information
#[derive(Debug, Clone)]
struct ClientConnection {
    id: String,
    client_type: ClientType,
    subscriptions: Vec<String>,
    last_ping: chrono::DateTime<chrono::Utc>,
}

/// Type of client connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientType {
    Dashboard { dashboard_id: String },
    Admin,
    Monitor,
    AlertReceiver,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WSMessage {
    // Client -> Server
    Subscribe { metrics: Vec<String> },
    Unsubscribe { metrics: Vec<String> },
    GetMetrics { metrics: Vec<String>, time_range_seconds: Option<u64> },
    CreateAlert { alert: AlertConfig },
    Ping,

    // Server -> Client
    MetricUpdate { data: MetricUpdate },
    DashboardUpdate { data: DashboardUpdate },
    AlertTriggered { data: AlertEvent },
    MetricsResponse { metrics: HashMap<String, Vec<crate::real_time_monitor::DataPoint>> },
    Pong,
    Error { message: String },
    Connected { client_id: String },
}

/// Alert configuration from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub name: String,
    pub metric_name: String,
    pub condition: String, // JSON string for condition
    pub severity: String,
    pub description: String,
}

/// Query parameters for WebSocket connection
#[derive(Debug, Deserialize)]
pub struct WSQuery {
    pub client_type: Option<String>,
    pub dashboard_id: Option<String>,
}

impl WebSocketService {
    pub fn new(monitor: Arc<RealTimeMonitor>) -> Self {
        Self {
            monitor,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create WebSocket router
    pub fn create_router(self) -> Router<Arc<RealTimeMonitor>> {
        let service = Arc::new(self);

        Router::new()
            .route("/ws", get(websocket_handler))
            .route("/ws/dashboard/:dashboard_id", get(dashboard_websocket_handler))
            .with_state(service)
    }

    /// Handle WebSocket connection
    pub async fn handle_websocket(
        &self,
        socket: WebSocket,
        client_type: ClientType,
    ) -> Result<()> {
        let client_id = Uuid::new_v4().to_string();

        // Register connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(client_id.clone(), ClientConnection {
                id: client_id.clone(),
                client_type: client_type.clone(),
                subscriptions: Vec::new(),
                last_ping: chrono::Utc::now(),
            });
        }

        tracing::info!("WebSocket client connected: {} ({:?})", client_id, client_type);

        // Split socket into sender and receiver
        let (mut sender, mut receiver) = socket.split();

        // Send welcome message
        let welcome_msg = WSMessage::Connected { client_id: client_id.clone() };
        if let Ok(msg_str) = serde_json::to_string(&welcome_msg) {
            let _ = sender.send(Message::Text(msg_str)).await;
        }

        // Create dashboard stream if it's a dashboard client
        let mut dashboard_rx = match &client_type {
            ClientType::Dashboard { dashboard_id } => {
                Some(self.monitor.create_dashboard_stream(
                    dashboard_id.clone(),
                    vec!["system.cpu.usage_percent".to_string(),
                         "system.memory.usage_percent".to_string(),
                         "aion.api.requests_per_second".to_string()]
                ).await)
            }
            _ => None,
        };

        // Subscribe to alert events
        let mut alert_rx = {
            // This would be connected to the alert event bus
            let (tx, rx) = broadcast::channel(100);
            rx
        };

        // Handle incoming messages
        let connections_for_receiver = Arc::clone(&self.connections);
        let monitor_for_receiver = Arc::clone(&self.monitor);
        let client_id_for_receiver = client_id.clone();

        tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(ws_msg) = serde_json::from_str::<WSMessage>(&text) {
                            Self::handle_client_message(
                                &connections_for_receiver,
                                &monitor_for_receiver,
                                &client_id_for_receiver,
                                ws_msg,
                            ).await;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("WebSocket client disconnected: {}", client_id_for_receiver);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error for client {}: {}", client_id_for_receiver, e);
                        break;
                    }
                    _ => {}
                }
            }

            // Remove connection
            let mut connections = connections_for_receiver.write().await;
            connections.remove(&client_id_for_receiver);
        });

        // Handle outgoing messages
        let client_id_for_sender = client_id.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Dashboard updates
                    dashboard_update = async {
                        if let Some(ref mut rx) = dashboard_rx {
                            rx.recv().await.ok()
                        } else {
                            None
                        }
                    } => {
                        if let Some(update) = dashboard_update {
                            let msg = WSMessage::DashboardUpdate { data: update };
                            if let Ok(msg_str) = serde_json::to_string(&msg) {
                                if sender.send(Message::Text(msg_str)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }

                    // Alert events
                    alert_event = alert_rx.recv() => {
                        if let Ok(event) = alert_event {
                            let msg = WSMessage::AlertTriggered { data: event };
                            if let Ok(msg_str) = serde_json::to_string(&msg) {
                                if sender.send(Message::Text(msg_str)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }

                    // Periodic ping
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(30)) => {
                        let ping_msg = WSMessage::Ping;
                        if let Ok(msg_str) = serde_json::to_string(&ping_msg) {
                            if sender.send(Message::Text(msg_str)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            }

            tracing::info!("WebSocket sender task ended for client: {}", client_id_for_sender);
        });

        Ok(())
    }

    /// Handle client message
    async fn handle_client_message(
        connections: &Arc<RwLock<HashMap<String, ClientConnection>>>,
        monitor: &Arc<RealTimeMonitor>,
        client_id: &str,
        message: WSMessage,
    ) {
        match message {
            WSMessage::Subscribe { metrics } => {
                let mut connections_guard = connections.write().await;
                if let Some(connection) = connections_guard.get_mut(client_id) {
                    connection.subscriptions.extend(metrics);
                    tracing::debug!("Client {} subscribed to metrics: {:?}", client_id, connection.subscriptions);
                }
            }

            WSMessage::Unsubscribe { metrics } => {
                let mut connections_guard = connections.write().await;
                if let Some(connection) = connections_guard.get_mut(client_id) {
                    connection.subscriptions.retain(|m| !metrics.contains(m));
                    tracing::debug!("Client {} unsubscribed from metrics", client_id);
                }
            }

            WSMessage::GetMetrics { metrics, time_range_seconds } => {
                let time_range = time_range_seconds.map(|s| std::time::Duration::from_secs(s));
                if let Ok(metric_data) = monitor.get_metrics(&metrics, time_range).await {
                    // Would send response back to client
                    tracing::debug!("Retrieved metrics for client {}: {} series", client_id, metric_data.len());
                }
            }

            WSMessage::CreateAlert { alert } => {
                // Parse alert condition and create alert
                tracing::info!("Client {} requested alert creation: {}", client_id, alert.name);
                // Implementation would parse the alert and register it
            }

            WSMessage::Ping => {
                let mut connections_guard = connections.write().await;
                if let Some(connection) = connections_guard.get_mut(client_id) {
                    connection.last_ping = chrono::Utc::now();
                }
            }

            _ => {
                tracing::warn!("Unexpected message from client {}: {:?}", client_id, message);
            }
        }
    }

    /// Get connected clients count
    pub async fn get_connected_clients(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }

    /// Get clients by type
    pub async fn get_clients_by_type(&self, client_type: &ClientType) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.values()
            .filter(|conn| std::mem::discriminant(&conn.client_type) == std::mem::discriminant(client_type))
            .map(|conn| conn.id.clone())
            .collect()
    }

    /// Broadcast message to all clients of a specific type
    pub async fn broadcast_to_type(&self, client_type: &ClientType, message: WSMessage) {
        let connections = self.connections.read().await;
        let message_str = match serde_json::to_string(&message) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("Failed to serialize WebSocket message: {}", e);
                return;
            }
        };

        for connection in connections.values() {
            if std::mem::discriminant(&connection.client_type) == std::mem::discriminant(client_type) {
                // In a real implementation, we'd send the message to the client
                tracing::debug!("Broadcasting message to client: {}", connection.id);
            }
        }
    }
}

/// WebSocket handler for general connections
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(service): State<Arc<WebSocketService>>,
    Query(params): Query<WSQuery>,
) -> Response {
    let client_type = match params.client_type.as_deref() {
        Some("admin") => ClientType::Admin,
        Some("monitor") => ClientType::Monitor,
        Some("alerts") => ClientType::AlertReceiver,
        Some("dashboard") => {
            if let Some(dashboard_id) = params.dashboard_id {
                ClientType::Dashboard { dashboard_id }
            } else {
                ClientType::Monitor
            }
        }
        _ => ClientType::Monitor,
    };

    ws.on_upgrade(move |socket| async move {
        if let Err(e) = service.handle_websocket(socket, client_type).await {
            tracing::error!("WebSocket connection error: {}", e);
        }
    })
}

/// WebSocket handler for dashboard-specific connections
async fn dashboard_websocket_handler(
    ws: WebSocketUpgrade,
    State(service): State<Arc<WebSocketService>>,
    Path(dashboard_id): Path<String>,
) -> Response {
    let client_type = ClientType::Dashboard { dashboard_id };

    ws.on_upgrade(move |socket| async move {
        if let Err(e) = service.handle_websocket(socket, client_type).await {
            tracing::error!("Dashboard WebSocket connection error: {}", e);
        }
    })
}

/// Health check for WebSocket service
pub async fn websocket_health_check(service: &WebSocketService) -> Result<serde_json::Value> {
    let connected_clients = service.get_connected_clients().await;
    let admin_clients = service.get_clients_by_type(&ClientType::Admin).await.len();
    let monitor_clients = service.get_clients_by_type(&ClientType::Monitor).await.len();

    Ok(serde_json::json!({
        "status": "healthy",
        "connected_clients": connected_clients,
        "admin_clients": admin_clients,
        "monitor_clients": monitor_clients,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}