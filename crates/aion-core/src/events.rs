use crate::PlatformConfig;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: &PlatformEvent) -> Result<()>;
    fn event_types(&self) -> Vec<&'static str>;
    fn handler_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformEvent {
    PlatformStarted { timestamp: chrono::DateTime<chrono::Utc>, services_count: usize },
    PlatformStopped { timestamp: chrono::DateTime<chrono::Utc>, uptime_seconds: u64 },
    ServiceRegistered { service_name: String, timestamp: chrono::DateTime<chrono::Utc> },
    ServiceStarted { service_name: String, timestamp: chrono::DateTime<chrono::Utc> },
    ServiceStopped { service_name: String, timestamp: chrono::DateTime<chrono::Utc> },
    ServiceError { service_name: String, error: String, timestamp: chrono::DateTime<chrono::Utc> },
    UserLoggedIn { user_id: uuid::Uuid, timestamp: chrono::DateTime<chrono::Utc> },
    DataProcessed { bytes: u64, timestamp: chrono::DateTime<chrono::Utc> },
    AlertTriggered { alert_id: String, severity: String, timestamp: chrono::DateTime<chrono::Utc> },
    Custom { event_type: String, data: serde_json::Value, timestamp: chrono::DateTime<chrono::Utc> },
}

pub struct EventBus {
    config: Arc<PlatformConfig>,
    handlers: Arc<RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>>,
    event_sender: broadcast::Sender<PlatformEvent>,
    _event_receiver: broadcast::Receiver<PlatformEvent>,
}

impl EventBus {
    pub async fn new(config: Arc<PlatformConfig>) -> Result<Self> {
        let (event_sender, event_receiver) = broadcast::channel(1000);

        Ok(Self {
            config,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            _event_receiver: event_receiver,
        })
    }

    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) -> Result<()> {
        let mut handlers = self.handlers.write().await;

        for event_type in handler.event_types() {
            handlers
                .entry(event_type.to_string())
                .or_insert_with(Vec::new)
                .push(handler.clone());
        }

        tracing::info!("Registered event handler: {}", handler.handler_name());
        Ok(())
    }

    pub async fn emit_event(&self, event: PlatformEvent) -> Result<()> {
        // Send to broadcast channel
        if let Err(e) = self.event_sender.send(event.clone()) {
            tracing::warn!("Failed to broadcast event: {}", e);
        }

        // Send to registered handlers
        let handlers = self.handlers.read().await;
        let event_type = match &event {
            PlatformEvent::PlatformStarted { .. } => "PlatformStarted",
            PlatformEvent::PlatformStopped { .. } => "PlatformStopped",
            PlatformEvent::ServiceRegistered { .. } => "ServiceRegistered",
            PlatformEvent::ServiceStarted { .. } => "ServiceStarted",
            PlatformEvent::ServiceStopped { .. } => "ServiceStopped",
            PlatformEvent::ServiceError { .. } => "ServiceError",
            PlatformEvent::UserLoggedIn { .. } => "UserLoggedIn",
            PlatformEvent::DataProcessed { .. } => "DataProcessed",
            PlatformEvent::AlertTriggered { .. } => "AlertTriggered",
            PlatformEvent::Custom { event_type, .. } => event_type,
        };

        if let Some(event_handlers) = handlers.get(event_type) {
            for handler in event_handlers {
                if let Err(e) = handler.handle_event(&event).await {
                    tracing::error!("Event handler {} failed: {}", handler.handler_name(), e);
                }
            }
        }

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<PlatformEvent> {
        self.event_sender.subscribe()
    }
}