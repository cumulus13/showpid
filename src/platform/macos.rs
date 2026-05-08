//! macOS window activation implementation
//!
//! Uses AppleScript (osascript) to bring process to foreground.

use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::config::Config;
use crate::error::{Result, ShowpidError};
use crate::platform::ActivateWindow;
use crate::window::WindowInfo;

/// macOS window activator
pub struct WindowActivator {
    config: Config,
}

impl WindowActivator {
    /// Create a new macOS activator
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ActivateWindow for WindowActivator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn find_windows(&mut self) -> Result<Vec<WindowInfo>> {
        // macOS uses osascript directly; just return a placeholder
        Ok(vec![WindowInfo::new_macos(self.config.pid, None, 0)])
    }

    fn activate(&self, _window: &WindowInfo) -> Result<()> {
        let script = format!(
            "tell application \"System Events\" to set frontmost of every process whose unix id is {} to true",
            self.config.pid
        );

        let output = Command::new("osascript").args(["-e", &script]).output().map_err(|e| {
            ShowpidError::PlatformError { platform: "macOS".into(), message: format!("Failed to run osascript: {}", e) }
        })?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ShowpidError::PlatformError {
                platform: "macOS".into(),
                message: format!("osascript error: {}", stderr.trim()),
            })
        }
    }

    fn execute(&mut self) -> Result<()> {
        for attempt in 0..self.config.retries {
            if attempt > 0 {
                if self.config.verbose {
                    eprintln!(
                        "[RETRY] {}/{} after {}ms",
                        attempt + 1,
                        self.config.retries,
                        self.config.retry_delay.as_millis()
                    );
                }
                thread::sleep(self.config.retry_delay);
            }

            let windows = self.find_windows()?;

            for w in &windows {
                match self.activate(w) {
                    Ok(()) => {
                        if self.config.verbose {
                            eprintln!("[SUCCESS] Activated PID {}", self.config.pid);
                        }
                        return Ok(());
                    }
                    Err(e) => {
                        if self.config.verbose {
                            eprintln!("[WARN] {}", e);
                        }
                    }
                }
            }
        }

        Err(ShowpidError::NoWindowFound {
            pid: self.config.pid,
            attempts: self.config.retries,
            message: "Failed to activate via osascript".into(),
        })
    }
}
