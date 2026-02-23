//! CSRF Protection Service
//!
//! Generates and validates per-session CSRF tokens using HMAC-SHA256.
//! Tokens are embedded in HTML forms/HTMX headers and validated on every
//! state-changing request (POST, PUT, PATCH, DELETE).
//!
//! Security properties:
//! - Tokens are tied to the session cookie (cannot be reused across sessions)
//! - Double-submit cookie pattern: token in cookie + token in request header/form
//! - Constant-time comparison prevents timing attacks

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// CSRF token length in bytes (32 bytes = 256 bits)
const TOKEN_BYTES: usize = 32;

/// Secret key for HMAC signing (generated once at startup)
#[derive(Clone)]
pub struct CsrfSecret(Vec<u8>);

impl CsrfSecret {
    /// Generate a new random secret at server startup
    pub fn generate() -> Self {
        let mut key = vec![0u8; 64];
        rand::thread_rng().fill_bytes(&mut key);
        Self(key)
    }

    /// Generate a CSRF token bound to a session ID
    pub fn generate_token(&self, session_id: &str) -> String {
        // Random nonce
        let mut nonce = vec![0u8; TOKEN_BYTES];
        rand::thread_rng().fill_bytes(&mut nonce);

        // HMAC: SHA256(secret + session_id + nonce)
        let mut hasher = Sha256::new();
        hasher.update(&self.0);
        hasher.update(session_id.as_bytes());
        hasher.update(&nonce);
        let signature = hasher.finalize();

        // Encode as: nonce.signature (both base64url)
        let nonce_b64 = URL_SAFE_NO_PAD.encode(&nonce);
        let sig_b64 = URL_SAFE_NO_PAD.encode(signature);
        format!("{}.{}", nonce_b64, sig_b64)
    }

    /// Validate a CSRF token against a session ID (constant-time)
    pub fn validate_token(&self, token: &str, session_id: &str) -> bool {
        let parts: Vec<&str> = token.splitn(2, '.').collect();
        if parts.len() != 2 {
            return false;
        }

        let nonce = match URL_SAFE_NO_PAD.decode(parts[0]) {
            Ok(n) if n.len() == TOKEN_BYTES => n,
            _ => return false,
        };

        let provided_sig = match URL_SAFE_NO_PAD.decode(parts[1]) {
            Ok(s) => s,
            _ => return false,
        };

        // Recompute expected signature
        let mut hasher = Sha256::new();
        hasher.update(&self.0);
        hasher.update(session_id.as_bytes());
        hasher.update(&nonce);
        let expected_sig = hasher.finalize();

        // Constant-time comparison
        constant_time_eq(&provided_sig, &expected_sig)
    }
}

/// Constant-time byte comparison to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate() {
        let secret = CsrfSecret::generate();
        let session = "test-session-123";
        let token = secret.generate_token(session);

        assert!(secret.validate_token(&token, session));
        assert!(!secret.validate_token(&token, "wrong-session"));
        assert!(!secret.validate_token("garbage", session));
    }

    #[test]
    fn test_tokens_are_unique() {
        let secret = CsrfSecret::generate();
        let t1 = secret.generate_token("session");
        let t2 = secret.generate_token("session");
        assert_ne!(t1, t2); // Different nonces
    }
}
