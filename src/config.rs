//! Configuration management for showpid

use std::time::Duration;

/// Default number of retry attempts if window not found
pub const DEFAULT_RETRIES: u32 = 3;

/// Default delay between retries in milliseconds
pub const DEFAULT_RETRY_DELAY_MS: u64 = 100;

/// Application configuration
///
/// # Examples
///
/// ```rust
/// use showpid::Config;
/// use std::time::Duration;
///
/// let config = Config::new(1234)
///     .with_verbose(true)
///     .with_retries(5)
///     .with_retry_delay(Duration::from_millis(200));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Process ID to search for
    pub pid: u32,

    /// Number of retry attempts before giving up
    pub retries: u32,

    /// Delay between retry attempts
    pub retry_delay: Duration,

    /// Enable verbose output
    pub verbose: bool,
}

impl Config {
    /// Create a new Config with default values
    ///
    /// # Arguments
    /// * `pid` - The process ID to target
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            retries: DEFAULT_RETRIES,
            retry_delay: Duration::from_millis(DEFAULT_RETRY_DELAY_MS),
            verbose: false,
        }
    }

    /// Enable verbose output mode
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set the number of retry attempts
    ///
    /// # Panics
    /// Panics if retries is 0
    pub fn with_retries(mut self, retries: u32) -> Self {
        assert!(retries > 0, "Retries must be greater than 0");
        self.retries = retries;
        self
    }

    /// Set the delay between retries
    pub fn with_retry_delay(mut self, delay: Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.pid == 0 {
            return Err("PID must be greater than 0".to_string());
        }
        if self.retries == 0 {
            return Err("Retries must be greater than 0".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new(1234);
        assert_eq!(config.pid, 1234);
        assert_eq!(config.retries, DEFAULT_RETRIES);
        assert!(!config.verbose);
    }

    #[test]
    fn test_builder_pattern() {
        let config = Config::new(5678).with_verbose(true).with_retries(5).with_retry_delay(Duration::from_millis(200));

        assert!(config.verbose);
        assert_eq!(config.retries, 5);
        assert_eq!(config.retry_delay, Duration::from_millis(200));
    }

    #[test]
    fn test_validation() {
        assert!(Config::new(1).validate().is_ok());
        assert!(Config::new(0).validate().is_err());
    }

    #[test]
    #[should_panic]
    fn test_zero_retries_panics() {
        Config::new(1234).with_retries(0);
    }
}
