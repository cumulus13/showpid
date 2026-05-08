//! Windows-specific window activation implementation
//!
//! Uses Win32 API to enumerate, find, and activate windows.

use std::thread;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, SetForegroundWindow,
    ShowWindow, SW_RESTORE,
};

use crate::config::Config;
use crate::error::{Result, ShowpidError};
use crate::platform::ActivateWindow;
use crate::window::WindowInfo;

/// Windows window activator
pub struct WindowActivator {
    config: Config,
}

impl WindowActivator {
    /// Create a new Windows activator
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get the title text of a window by handle
    fn window_title(hwnd: HWND) -> Option<String> {
        unsafe {
            let len = GetWindowTextLengthW(hwnd);
            if len == 0 {
                return None;
            }
            let mut buf = vec![0u16; (len + 1) as usize];
            let copied = GetWindowTextW(hwnd, &mut buf);
            if copied == 0 {
                return None;
            }
            buf.truncate(copied as usize);
            String::from_utf16(&buf).ok()
        }
    }

    /// EnumWindows callback to collect visible window handles
    extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        unsafe {
            if IsWindowVisible(hwnd).as_bool() {
                let list = &mut *(lparam.0 as *mut Vec<HWND>);
                list.push(hwnd);
            }
        }
        TRUE
    }

    /// Enumerate all visible windows on the system
    fn enumerate_visible_windows() -> Vec<HWND> {
        let mut hwnds: Vec<HWND> = Vec::new();
        unsafe {
            let _ = EnumWindows(Some(Self::enum_callback), LPARAM(&mut hwnds as *mut Vec<HWND> as isize));
        }
        hwnds
    }
}

impl ActivateWindow for WindowActivator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn find_windows(&mut self) -> Result<Vec<WindowInfo>> {
        let all = Self::enumerate_visible_windows();
        let mut result = Vec::new();

        for hwnd in all {
            unsafe {
                let mut pid: u32 = 0;
                let tid = GetWindowThreadProcessId(hwnd, Some(&mut pid));
                if tid != 0 && pid == self.config.pid {
                    let title = Self::window_title(hwnd);
                    // HWND wraps *mut c_void, use hwnd.0 to get the raw pointer
                    result.push(WindowInfo::new_windows(pid, title, hwnd.0));
                }
            }
        }

        Ok(result)
    }

    fn activate(&self, window: &WindowInfo) -> Result<()> {
        // Extract HWND from window handle (always Win32 on Windows)
        let crate::window::WindowHandle::Win32(raw_ptr) = window.handle;

        // Reconstruct HWND from raw pointer
        let hwnd = HWND(raw_ptr);

        unsafe {
            // Restore window if minimized
            let _ = ShowWindow(hwnd, SW_RESTORE);

            // Attach thread input for reliable cross-process activation
            let mut pid: u32 = 0;
            let target_tid = GetWindowThreadProcessId(hwnd, Some(&mut pid));
            let current_tid = GetCurrentThreadId();

            if target_tid != 0 && target_tid != current_tid {
                let _ = AttachThreadInput(current_tid, target_tid, TRUE);
            }

            let ok = SetForegroundWindow(hwnd);

            if target_tid != 0 && target_tid != current_tid {
                let _ = AttachThreadInput(current_tid, target_tid, BOOL(0));
            }

            if ok.as_bool() {
                Ok(())
            } else {
                Err(ShowpidError::PlatformError {
                    platform: "Windows".into(),
                    message: format!("SetForegroundWindow failed for '{}'", window.title_or_unknown()),
                })
            }
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

            if windows.is_empty() {
                if self.config.verbose {
                    eprintln!("[INFO] No visible windows for PID {} (attempt {})", self.config.pid, attempt + 1);
                }
                continue;
            }

            if self.config.verbose {
                eprintln!("[FOUND] {} window(s) for PID {}", windows.len(), self.config.pid);
                for w in &windows {
                    eprintln!("  - {}", w);
                }
            }

            for w in &windows {
                match self.activate(w) {
                    Ok(()) => {
                        if self.config.verbose {
                            eprintln!("[SUCCESS] Activated {}", w);
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
            message: "No visible windows found".into(),
        })
    }
}
