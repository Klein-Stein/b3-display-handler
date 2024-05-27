//! This module contains all public exports of the b3-display-handler crate.

#![warn(missing_docs)]

use appkit::AppKitWindowHandler;

pub mod appkit;

/// Provider of a raw pointer to a system window.
pub trait HasWindowHandler {
    /// Returns a window handler for the current platform.
    fn window_handler(&self) -> WindowHandler;
}

/// Window handler.
///
/// The window handler provides raw pointers for native system windows.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowHandler {
    /// Window handler implementation for AppKit.
    AppKit(AppKitWindowHandler),
}
