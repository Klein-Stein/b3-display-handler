//! Implementations for AppKit

use std::{ffi::c_void, ptr::NonNull};

/// Window hanlder implementation for AppKit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitWindowHandler {
    /// A pointer to an `NSView` instance.
    pub view_ptr: NonNull<c_void>,
}

impl AppKitWindowHandler {
    /// Creates a new instance of [AppKitWindowHandler].
    ///
    /// # Parameters:
    /// * `view_ptr` - A pointer to an `NSView` instance.
    pub fn new(view_ptr: NonNull<c_void>) -> Self {
        Self {
            view_ptr,
        }
    }
}
