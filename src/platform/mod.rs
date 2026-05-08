//! Platform-specific window activation implementations

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::WindowActivator;
#[cfg(target_os = "macos")]
pub use self::macos::WindowActivator;
#[cfg(windows)]
pub use self::windows::WindowActivator;

use crate::config::Config;
use crate::error::Result;
use crate::window::WindowInfo;

/// Common trait for platform-specific window activation
pub trait ActivateWindow {
    /// Find all windows for the configured PID
    fn find_windows(&mut self) -> Result<Vec<WindowInfo>>;

    /// Activate a specific window (bring to foreground)
    fn activate(&self, window: &WindowInfo) -> Result<()>;

    /// Main execution with retry logic
    fn execute(&mut self) -> Result<()>;

    /// Get the current configuration
    fn config(&self) -> &Config;
}
