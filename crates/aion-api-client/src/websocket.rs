use crate::{types::*, error::*};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use std::collections::HashMap;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct WebSocketClient {
    sender: mpsc::UnboundedSender<WebSocketCommand>,
    event_receiver: broadcast::Receiver<ProgressEvent>,
}

#[derive(Debug)]
enum WebSocketCommand {
    Subscribe { session_id: Uuid },
    Unsubscribe { session_id: Uuid },
    SendEvent { event: ProgressEvent },
    Close,
}

impl WebSocketClient {
    /// Create a new WebSocket client connection
    pub async fn new(base_url: &Url, api_key: &str) -> Result<Self> {
        let ws_url = Self::build_ws_url(base_url, api_key)?;

        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        let (cmd_sender, mut cmd_receiver) = mpsc::unbounded_channel::<WebSocketCommand>();
        let (event_sender, event_receiver) = broadcast::channel::<ProgressEvent>(1000);

        // Spawn the WebSocket handler task
        tokio::spawn(async move {
            let mut subscriptions: HashMap<Uuid, bool> = HashMap::new();

            loop {
                tokio::select! {
                    // Handle incoming WebSocket messages
                    ws_msg = ws_stream.next() => {
                        match ws_msg {
                            Some(Ok(Message::Text(text))) => {
                                match serde_json::from_str::<ProgressEvent>(&text) {
                                    Ok(event) => {
                                        if let Err(_) = event_sender.send(event) {
                                            tracing::warn!("No active listeners for progress events");
                                        }
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to parse WebSocket message: {}", e);
                                    }
                                }
                            }
                            Some(Ok(Message::Close(_))) => {
                                tracing::info!("WebSocket connection closed by server");
                                break;
                            }
                            Some(Err(e)) => {
                                tracing::error!("WebSocket error: {}", e);
                                break;
                            }
                            None => {
                                tracing::info!("WebSocket stream ended");
                                break;
                            }
                            _ => {} // Ignore other message types
                        }
                    }

                    // Handle commands from the client
                    cmd = cmd_receiver.recv() => {
                        match cmd {
                            Some(WebSocketCommand::Subscribe { session_id }) => {
                                subscriptions.insert(session_id, true);
                                let subscribe_msg = serde_json::json!({
                                    "action": "subscribe",
                                    "session_id": session_id
                                });
                                if let Err(e) = ws_sink.send(Message::Text(subscribe_msg.to_string())).await {
                                    tracing::error!("Failed to send subscribe message: {}", e);
                                }
                            }
                            Some(WebSocketCommand::Unsubscribe { session_id }) => {
                                subscriptions.remove(&session_id);
                                let unsubscribe_msg = serde_json::json!({
                                    "action": "unsubscribe",
                                    "session_id": session_id
                                });
                                if let Err(e) = ws_sink.send(Message::Text(unsubscribe_msg.to_string())).await {
                                    tracing::error!("Failed to send unsubscribe message: {}", e);
                                }
                            }
                            Some(WebSocketCommand::SendEvent { event }) => {
                                let event_msg = serde_json::json!({
                                    "action": "send_event",
                                    "event": event
                                });
                                if let Err(e) = ws_sink.send(Message::Text(event_msg.to_string())).await {
                                    tracing::error!("Failed to send event: {}", e);
                                }
                            }
                            Some(WebSocketCommand::Close) => {
                                if let Err(e) = ws_sink.close().await {
                                    tracing::error!("Failed to close WebSocket: {}", e);
                                }
                                break;
                            }
                            None => {
                                tracing::info!("Command channel closed");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            sender: cmd_sender,
            event_receiver,
        })
    }

    /// Build WebSocket URL from base URL and API key
    fn build_ws_url(base_url: &Url, api_key: &str) -> Result<String> {
        let mut ws_url = base_url.clone();

        // Convert HTTP(S) scheme to WS(S)
        let ws_scheme = match ws_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            scheme => return Err(AionError::Url(url::ParseError::InvalidIpv4Address)),
        };

        ws_url.set_scheme(ws_scheme)
            .map_err(|_| AionError::Url(url::ParseError::InvalidIpv4Address))?;

        // Set WebSocket path
        ws_url.set_path("/ws/progress");

        // Add API key as query parameter
        ws_url.set_query(Some(&format!("api_key={}", api_key)));

        Ok(ws_url.to_string())
    }

    /// Subscribe to progress events for a specific session
    pub async fn subscribe(&self, session_id: Uuid) -> Result<()> {
        self.sender
            .send(WebSocketCommand::Subscribe { session_id })
            .map_err(|_| AionError::Connection("WebSocket connection closed".to_string()))?;
        Ok(())
    }

    /// Unsubscribe from progress events for a specific session
    pub async fn unsubscribe(&self, session_id: Uuid) -> Result<()> {
        self.sender
            .send(WebSocketCommand::Unsubscribe { session_id })
            .map_err(|_| AionError::Connection("WebSocket connection closed".to_string()))?;
        Ok(())
    }

    /// Send a progress event through the WebSocket
    pub async fn send_event(&self, event: ProgressEvent) -> Result<()> {
        self.sender
            .send(WebSocketCommand::SendEvent { event })
            .map_err(|_| AionError::Connection("WebSocket connection closed".to_string()))?;
        Ok(())
    }

    /// Get a receiver for progress events
    pub fn events(&self) -> broadcast::Receiver<ProgressEvent> {
        self.event_receiver.resubscribe()
    }

    /// Close the WebSocket connection
    pub async fn close(&self) -> Result<()> {
        self.sender
            .send(WebSocketCommand::Close)
            .map_err(|_| AionError::Connection("WebSocket connection already closed".to_string()))?;
        Ok(())
    }

    /// Create a filtered event stream for a specific session
    pub fn session_events(&self, session_id: Uuid) -> SessionEventStream {
        SessionEventStream::new(self.event_receiver.resubscribe(), session_id)
    }

    /// Create a filtered event stream for specific event types
    pub fn typed_events(&self, event_types: Vec<ProgressEventType>) -> TypedEventStream {
        TypedEventStream::new(self.event_receiver.resubscribe(), event_types)
    }
}

/// Stream of events filtered by session ID
pub struct SessionEventStream {
    receiver: broadcast::Receiver<ProgressEvent>,
    session_id: Uuid,
}

impl SessionEventStream {
    fn new(receiver: broadcast::Receiver<ProgressEvent>, session_id: Uuid) -> Self {
        Self { receiver, session_id }
    }

    /// Get the next event for this session
    pub async fn next(&mut self) -> Option<ProgressEvent> {
        loop {
            match self.receiver.recv().await {
                Ok(event) if event.session_id == self.session_id => return Some(event),
                Ok(_) => continue, // Event for different session
                Err(broadcast::error::RecvError::Closed) => return None,
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    tracing::warn!("Event stream lagged, some events may have been missed");
                    continue;
                }
            }
        }
    }
}

/// Stream of events filtered by event type
pub struct TypedEventStream {
    receiver: broadcast::Receiver<ProgressEvent>,
    event_types: Vec<ProgressEventType>,
}

impl TypedEventStream {
    fn new(receiver: broadcast::Receiver<ProgressEvent>, event_types: Vec<ProgressEventType>) -> Self {
        Self { receiver, event_types }
    }

    /// Get the next event of the specified types
    pub async fn next(&mut self) -> Option<ProgressEvent> {
        loop {
            match self.receiver.recv().await {
                Ok(event) if self.event_types.contains(&event.event_type) => return Some(event),
                Ok(_) => continue, // Event of different type
                Err(broadcast::error::RecvError::Closed) => return None,
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    tracing::warn!("Event stream lagged, some events may have been missed");
                    continue;
                }
            }
        }
    }
}

/// Event listener with automatic subscription management
pub struct ProgressListener {
    client: WebSocketClient,
    session_id: Uuid,
}

impl ProgressListener {
    /// Create a new progress listener for a session
    pub async fn new(base_url: &Url, api_key: &str, session_id: Uuid) -> Result<Self> {
        let client = WebSocketClient::new(base_url, api_key).await?;
        client.subscribe(session_id).await?;

        Ok(Self { client, session_id })
    }

    /// Get the next progress event for this session
    pub async fn next_event(&self) -> Option<ProgressEvent> {
        let mut stream = self.client.session_events(self.session_id);
        stream.next().await
    }

    /// Get all events as a stream
    pub fn event_stream(&self) -> SessionEventStream {
        self.client.session_events(self.session_id)
    }

    /// Wait for session completion
    pub async fn wait_for_completion(&self) -> Option<ProgressEvent> {
        let mut stream = self.client.typed_events(vec![
            ProgressEventType::SessionCompleted,
            ProgressEventType::SessionFailed,
        ]);

        loop {
            if let Some(event) = stream.next().await {
                if event.session_id == self.session_id {
                    return Some(event);
                }
            } else {
                return None;
            }
        }
    }
}

impl Drop for ProgressListener {
    fn drop(&mut self) {
        // Best effort unsubscribe when the listener is dropped
        let sender = self.client.sender.clone();
        let session_id = self.session_id;
        tokio::spawn(async move {
            let _ = sender.send(WebSocketCommand::Unsubscribe { session_id });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_url_building() {
        let base_url = Url::parse("https://api.example.com").unwrap();
        let ws_url = WebSocketClient::build_ws_url(&base_url, "test-key").unwrap();
        assert_eq!(ws_url, "wss://api.example.com/ws/progress?api_key=test-key");
    }

    #[tokio::test]
    async fn test_ws_url_building_http() {
        let base_url = Url::parse("http://localhost:8080").unwrap();
        let ws_url = WebSocketClient::build_ws_url(&base_url, "test-key").unwrap();
        assert_eq!(ws_url, "ws://localhost:8080/ws/progress?api_key=test-key");
    }
}