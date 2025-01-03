use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

pub mod settings {
    use super::*;

    // Static variables for various settings
    lazy_static::lazy_static! {
        // Mutex-protected for settings that might require more complex types
        static ref UNIT: Mutex<String> = Mutex::new(String::new());
    }

    // Atomic values for thread-safe primitive types
    static MAX_CONNECTIONS: AtomicUsize = AtomicUsize::new(10);
    static LOGGING_ENABLED: AtomicBool = AtomicBool::new(true);

    /// Set the username
    pub fn set_unit(name: &str) {
        let mut unit = UNIT.lock().unwrap();
        *unit = name.to_string();
    }

    /// Get the username
    pub fn get_unit() -> String {
        let unit = UNIT.lock().unwrap();
        unit.clone()
    }

    /// Clear the unit
    pub fn clear_unit() {
        let mut unit = UNIT.lock().unwrap();
        *unit = String::new(); // Clear the content
    }

    /// Set the maximum number of connections
    pub fn set_max_connections(value: usize) {
        MAX_CONNECTIONS.store(value, Ordering::SeqCst);
    }

    /// Get the maximum number of connections
    pub fn get_max_connections() -> usize {
        MAX_CONNECTIONS.load(Ordering::SeqCst)
    }

    /// Enable or disable logging
    pub fn set_logging_enabled(enabled: bool) {
        LOGGING_ENABLED.store(enabled, Ordering::SeqCst);
    }

    /// Check if logging is enabled
    pub fn is_logging_enabled() -> bool {
        LOGGING_ENABLED.load(Ordering::SeqCst)
    }
}
