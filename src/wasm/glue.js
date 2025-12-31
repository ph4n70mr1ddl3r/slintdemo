// Slint WASM Glue Code
// Handles browser-specific initialization for Slint WebAssembly module

console.log('[Slint WASM] Initializing glue code...');

export function slint_init() {
    console.log('[Slint WASM] Platform initialization requested');
    // Additional platform setup can go here if needed
}

export function slint_version() {
    return '1.14.0';
}

export function slint_create_app() {
    console.log('[Slint WASM] Creating app instance...');
    // App creation is handled by wasm-bindgen generated code
}

// Error handling wrapper
export function slint_handle_error(message) {
    console.error('[Slint WASM] Error:', message);
    return false;
}

// Feature detection
export function slint_supports_wasm() {
    return typeof WebAssembly === 'object';
}

export function slint_supports_threads() {
    // Check for SharedArrayBuffer support (needed for threading)
    try {
        return typeof SharedArrayBuffer !== 'undefined';
    } catch {
        return false;
    }
}

console.log('[Slint WASM] Glue code loaded');
