//! Health Service — Server health and status information

use std::time::SystemTime;

/// Health status information
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub uptime_formatted: String,
}

/// Health service trait — allows mocking in tests
pub trait HealthService: Send + Sync {
    fn get_status(&self) -> HealthStatus;
}

/// Default implementation using system time
pub struct DefaultHealthService {
    start_time: SystemTime,
}

impl DefaultHealthService {
    pub fn new(start_time: SystemTime) -> Self {
        Self { start_time }
    }
}

impl HealthService for DefaultHealthService {
    fn get_status(&self) -> HealthStatus {
        let uptime_seconds = SystemTime::now()
            .duration_since(self.start_time)
            .unwrap_or_default()
            .as_secs();

        HealthStatus {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds,
            uptime_formatted: format_uptime(uptime_seconds),
        }
    }
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if minutes > 0 { parts.push(format!("{}m", minutes)); }

    if parts.is_empty() { "< 1m".to_string() } else { parts.join(" ") }
}
