use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Plugin event for event-driven communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Plugin was loaded
    Loaded {
        plugin_id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Plugin was unloaded
    Unloaded {
        plugin_id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Plugin execution started
    ExecutionStarted {
        plugin_id: Uuid,
        execution_id: Uuid,
        function: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Plugin execution completed
    ExecutionCompleted {
        plugin_id: Uuid,
        execution_id: Uuid,
        success: bool,
        duration_ms: u64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Plugin error occurred
    Error {
        plugin_id: Uuid,
        error: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

/// Event bus for plugin event communication
pub struct EventBus {
    sender: tokio::sync::broadcast::Sender<PluginEvent>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(1000);
        Self { sender }
    }

    /// Emit an event
    pub async fn emit(&self, event: PluginEvent) {
        // Ignore send errors (no receivers is fine)
        let _ = self.sender.send(event);
    }

    /// Subscribe to events
    pub async fn subscribe(&self) -> tokio::sync::broadcast::Receiver<PluginEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
