//! Session Service — Secure cookie-based sessions
//!
//! Provides server-side session management with:
//! - Random 256-bit session IDs
//! - HttpOnly, Secure, SameSite=Strict cookies
//! - In-memory session store (swap for Redis/DB in production)
//! - Automatic cleanup of expired sessions

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Session cookie name — intentionally generic to avoid fingerprinting
pub const SESSION_COOKIE: &str = "__Host-sid";

/// Session lifetime
const SESSION_TTL: Duration = Duration::from_secs(3600); // 1 hour

/// Session data stored server-side
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub csrf_token: String,
    pub created_at: Instant,
    pub last_access: Instant,
    pub data: HashMap<String, String>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.last_access.elapsed() > SESSION_TTL
    }
}

/// Session store trait — allows swapping in-memory for Redis, DB, etc.
pub trait SessionStore: Send + Sync {
    fn create(&self) -> Session;
    fn get(&self, id: &str) -> Option<Session>;
    fn touch(&self, id: &str);
    fn update_csrf(&self, id: &str, token: &str);
    fn destroy(&self, id: &str);
    fn cleanup_expired(&self);
}

/// In-memory session store (suitable for single-instance deployments)
pub struct InMemorySessionStore {
    sessions: RwLock<HashMap<String, Session>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }

    fn generate_id() -> String {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(bytes)
    }
}

impl Default for InMemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore for InMemorySessionStore {
    fn create(&self) -> Session {
        let session = Session {
            id: Self::generate_id(),
            csrf_token: String::new(),
            created_at: Instant::now(),
            last_access: Instant::now(),
            data: HashMap::new(),
        };
        self.sessions
            .write()
            .unwrap()
            .insert(session.id.clone(), session.clone());
        session
    }

    fn get(&self, id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(id).filter(|s| !s.is_expired()).cloned()
    }

    fn touch(&self, id: &str) {
        if let Some(session) = self.sessions.write().unwrap().get_mut(id) {
            session.last_access = Instant::now();
        }
    }

    fn update_csrf(&self, id: &str, token: &str) {
        if let Some(session) = self.sessions.write().unwrap().get_mut(id) {
            session.csrf_token = token.to_string();
        }
    }

    fn destroy(&self, id: &str) {
        self.sessions.write().unwrap().remove(id);
    }

    fn cleanup_expired(&self) {
        self.sessions
            .write()
            .unwrap()
            .retain(|_, s| !s.is_expired());
    }
}
