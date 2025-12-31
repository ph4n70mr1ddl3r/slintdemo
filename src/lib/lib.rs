//! Slint Showcase Library
//! Main entry point for the Slint UI components

pub mod components;
pub mod data;

// Re-export components for easy access
pub use components::app::App;

use slint::{ComponentHandle, Platform};

/// Initialize the Slint platform for WebAssembly
#[cfg(target_arch = "wasm32")]
pub fn init_platform() {
    // WebAssembly platform is automatically initialized by wasm-bindgen
    console_error_panic_hook_set();
}

/// Set up better error messages for panic hooks
#[cfg(target_arch = "wasm32")]
fn console_error_panic_hook_set() {
    // Use console_error_panic_hook for better debugging
    // This is typically enabled via console_error_panic_hook crate
}

/// Create and return the main application component
#[cfg(target_arch = "wasm32")]
pub fn create_app() -> Result<App, slint::PlatformError> {
    App::new()
}

/// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Slint version being showcased
pub const SLINT_VERSION: &str = "1.14";
