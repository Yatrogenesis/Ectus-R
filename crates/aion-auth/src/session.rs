use crate::models::Session;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create_session(&self, session: Session) -> Result<()>;
    async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>>;
    async fn update_session(&self, session: Session) -> Result<()>;
    async fn delete_session(&self, session_id: Uuid) -> Result<()>;
    async fn cleanup_expired_sessions(&self) -> Result<()>;
}

pub struct InMemorySessionStore {
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SessionStore for InMemorySessionStore {
    async fn create_session(&self, session: Session) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
        Ok(())
    }

    async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(&session_id).cloned())
    }

    async fn update_session(&self, session: Session) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
        Ok(())
    }

    async fn delete_session(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);
        Ok(())
    }

    async fn cleanup_expired_sessions(&self) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();
        sessions.retain(|_, session| session.expires_at > now);
        Ok(())
    }
}

pub struct SessionManager {
    store: Arc<dyn SessionStore>,
}

impl SessionManager {
    pub fn new(store: Arc<dyn SessionStore>) -> Self {
        Self { store }
    }

    pub async fn create_session(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Session> {
        let session = Session {
            id: Uuid::new_v4(),
            user_id,
            tenant_id,
            token_hash: Uuid::new_v4().to_string(),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(8),
            last_activity: Utc::now(),
            is_active: true,
        };

        self.store.create_session(session.clone()).await?;
        Ok(session)
    }

    pub async fn validate_session(&self, session_id: Uuid) -> Result<Option<Session>> {
        if let Some(mut session) = self.store.get_session(session_id).await? {
            if session.expires_at > Utc::now() && session.is_active {
                // Update last activity
                session.last_activity = Utc::now();
                self.store.update_session(session.clone()).await?;
                Ok(Some(session))
            } else {
                // Session expired or inactive
                self.store.delete_session(session_id).await?;
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn invalidate_session(&self, session_id: Uuid) -> Result<()> {
        self.store.delete_session(session_id).await
    }
}