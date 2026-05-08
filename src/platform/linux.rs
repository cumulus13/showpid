//! Linux (X11) window activation implementation
//!
//! Uses X11 Xlib to enumerate, find, and activate windows.

use std::ffi::CString;
use std::ptr;
use std::thread;
use x11::xlib;

use crate::config::Config;
use crate::error::{Result, ShowpidError};
use crate::platform::ActivateWindow;
use crate::window::WindowInfo;

/// Linux (X11) window activator
pub struct WindowActivator {
    config: Config,
}

impl WindowActivator {
    /// Create a new Linux activator
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get PID from _NET_WM_PID property
    unsafe fn get_window_pid(dpy: *mut xlib::Display, win: xlib::Window) -> Option<u32> {
        let atom_name = CString::new("_NET_WM_PID").unwrap();
        let atom = xlib::XInternAtom(dpy, atom_name.as_ptr(), 0);
        let mut typ: xlib::Atom = 0;
        let mut fmt: i32 = 0;
        let mut nitems: u64 = 0;
        let mut after: u64 = 0;
        let mut prop: *mut u8 = ptr::null_mut();

        let rc = xlib::XGetWindowProperty(
            dpy,
            win,
            atom,
            0,
            1,
            0,
            xlib::XA_CARDINAL,
            &mut typ,
            &mut fmt,
            &mut nitems,
            &mut after,
            &mut prop,
        );

        if rc == xlib::Success as i32 && !prop.is_null() && nitems > 0 {
            let pid = *(prop as *const u32);
            xlib::XFree(prop as *mut _);
            Some(pid)
        } else {
            if !prop.is_null() {
                xlib::XFree(prop as *mut _);
            }
            None
        }
    }

    /// Get window title via XFetchName
    unsafe fn get_window_title(dpy: *mut xlib::Display, win: xlib::Window) -> Option<String> {
        let mut name: *mut i8 = ptr::null_mut();
        if xlib::XFetchName(dpy, win, &mut name) != 0 && !name.is_null() {
            let s = std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned();
            xlib::XFree(name as *mut _);
            Some(s)
        } else {
            None
        }
    }
}

impl ActivateWindow for WindowActivator {
    fn config(&self) -> &Config {
        &self.config
    }

    fn find_windows(&mut self) -> Result<Vec<WindowInfo>> {
        let mut result = Vec::new();

        unsafe {
            let dpy = xlib::XOpenDisplay(ptr::null());
            if dpy.is_null() {
                return Err(ShowpidError::PlatformError {
                    platform: "Linux".into(),
                    message: "Cannot open X display".into(),
                });
            }

            let root = xlib::XDefaultRootWindow(dpy);
            let mut root_ret = 0;
            let mut parent_ret = 0;
            let mut children: *mut xlib::Window = ptr::null_mut();
            let mut n = 0u32;

            if xlib::XQueryTree(dpy, root, &mut root_ret, &mut parent_ret, &mut children, &mut n) != 0
                && !children.is_null()
            {
                let slice = std::slice::from_raw_parts(children, n as usize);
                for &child in slice {
                    if let Some(pid) = Self::get_window_pid(dpy, child) {
                        if pid == self.config.pid {
                            let title = Self::get_window_title(dpy, child);
                            result.push(WindowInfo::new_linux(pid, title, child));
                        }
                    }
                }
                xlib::XFree(children as *mut _);
            }

            xlib::XCloseDisplay(dpy);
        }

        Ok(result)
    }

    fn activate(&self, window: &WindowInfo) -> Result<()> {
        let xid = match &window.handle {
            crate::window::WindowHandle::X11(id) => *id,
            #[allow(unreachable_patterns)]
            _ => {
                return Err(ShowpidError::PlatformError {
                    platform: "Linux".into(),
                    message: "Invalid window handle".into(),
                })
            }
        };

        unsafe {
            let dpy = xlib::XOpenDisplay(ptr::null());
            if dpy.is_null() {
                return Err(ShowpidError::PlatformError {
                    platform: "Linux".into(),
                    message: "Cannot open X display".into(),
                });
            }

            xlib::XRaiseWindow(dpy, xid);
            xlib::XSetInputFocus(dpy, xid, xlib::RevertToParent, xlib::CurrentTime);
            xlib::XMapWindow(dpy, xid);
            xlib::XFlush(dpy);
            xlib::XCloseDisplay(dpy);
        }

        Ok(())
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
                    eprintln!("[INFO] No X11 windows for PID {} (attempt {})", self.config.pid, attempt + 1);
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
            message: "No X11 windows found".into(),
        })
    }
}
