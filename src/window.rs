//! Window information types

use std::fmt;

/// Platform-agnostic window information
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// Process ID associated with this window
    pub pid: u32,

    /// Window title text, if available
    pub title: Option<String>,

    /// Platform-specific window handle
    pub(crate) handle: WindowHandle,
}

/// Platform-specific window handle
#[derive(Debug, Clone)]
pub(crate) enum WindowHandle {
    /// Windows HWND (*mut c_void)
    #[cfg(windows)]
    Win32(*mut std::ffi::c_void),

    /// Linux X11 Window ID
    #[cfg(target_os = "linux")]
    X11(u64),

    /// macOS window number (CGWindowID)
    #[cfg(target_os = "macos")]
    CoreGraphics(u32),
}

impl WindowInfo {
    /// Create a new WindowInfo on Windows
    #[cfg(windows)]
    pub fn new_windows(pid: u32, title: Option<String>, hwnd: *mut std::ffi::c_void) -> Self {
        Self { pid, title, handle: WindowHandle::Win32(hwnd) }
    }

    /// Create a new WindowInfo on Linux
    #[cfg(target_os = "linux")]
    pub fn new_linux(pid: u32, title: Option<String>, xid: u64) -> Self {
        Self { pid, title, handle: WindowHandle::X11(xid) }
    }

    /// Create a new WindowInfo on macOS
    #[cfg(target_os = "macos")]
    pub fn new_macos(pid: u32, title: Option<String>, window_id: u32) -> Self {
        Self { pid, title, handle: WindowHandle::CoreGraphics(window_id) }
    }

    /// Get the window title or "Unknown" if not available
    pub fn title_or_unknown(&self) -> &str {
        self.title.as_deref().unwrap_or("Unknown")
    }
}

impl fmt::Display for WindowInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.title {
            Some(title) => write!(f, "Window(PID={}, '{}')", self.pid, title),
            None => write!(f, "Window(PID={})", self.pid),
        }
    }
}
