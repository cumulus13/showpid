//! # showpid
//!
//! Cross-platform library and CLI utility to bring a window to foreground by PID.
//!
//! Supports Windows, Linux (X11), and macOS.
//!
//! ## Library Usage
//!
//! ```rust
//! use showpid::Config;
//! use showpid::platform::{WindowActivator, ActivateWindow};
//!
//! let config = Config::new(1234).with_verbose(true);
//! let mut activator = WindowActivator::new(config);
//! // activator.execute()?;
//! ```

pub mod config;
pub mod error;
pub mod platform;
pub mod window;

pub use config::Config;
pub use error::{Result, ShowpidError};
pub use window::WindowInfo;
