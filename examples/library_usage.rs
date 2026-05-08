//! Example: Using showpid as a library
//!
//! Demonstrates programmatic window activation using the showpid API.

use showpid::config::Config;
use showpid::platform::ActivateWindow;
use showpid::platform::WindowActivator;

fn main() {
    // Configure for PID 1234 with verbose output
    let config = Config::new(1234).with_verbose(true);

    // Create platform-specific activator
    let mut activator = WindowActivator::new(config);

    // Execute window activation
    match activator.execute() {
        Ok(()) => println!("Window activated successfully!"),
        Err(e) => eprintln!("Failed to activate window: {}", e),
    }
}
