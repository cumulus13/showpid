//! Integration tests for showpid

use showpid::config::Config;
use showpid::platform::ActivateWindow;
use showpid::platform::WindowActivator;

#[test]
fn test_config_creation() {
    let config = Config::new(1234);
    assert_eq!(config.pid, 1234);
}

#[test]
fn test_config_validation() {
    assert!(Config::new(1).validate().is_ok());
    assert!(Config::new(0).validate().is_err());
}

#[test]
fn test_activator_creation() {
    let config = Config::new(1234);
    let activator = WindowActivator::new(config.clone());
    assert_eq!(activator.config().pid, 1234);
}
