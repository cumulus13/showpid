//! showpid CLI entry point
//!
//! Parses command-line arguments and executes the platform-specific
//! window activation logic.

use std::process;

use clap_version_flag::colorful_version;

use showpid::config::Config;
use showpid::platform::ActivateWindow;
use showpid::platform::WindowActivator;

/// Print usage information and exit
fn usage(prog: &str) -> ! {
    eprintln!("showpid v{} - Bring window to foreground by PID", env!("CARGO_PKG_VERSION"));
    eprintln!("Cross-platform: Windows, Linux, macOS\n");
    eprintln!("Usage:  {} [OPTIONS] <PID>\n", prog);
    eprintln!("Options:");
    eprintln!("  -v, --verbose   Detailed progress output");
    eprintln!("  -V, --version   Print version and exit");
    eprintln!("  -h, --help      This help message\n");
    eprintln!("Examples:");
    eprintln!("  {} 1234         Bring PID 1234 to foreground", prog);
    eprintln!("  {} -v 5678      Verbose mode", prog);
    process::exit(1);
}

/// Print version and exit
fn version() -> ! {
    println!("showpid v{}", env!("CARGO_PKG_VERSION"));
    println!("Author : {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    process::exit(0);
}

/// Parse PID from command-line argument string
fn parse_pid(arg: &str) -> Result<u32, String> {
    let pid: u32 = arg.parse().map_err(|e| format!("Invalid PID '{}': {}", arg, e))?;
    if pid == 0 {
        return Err("PID must be greater than 0".to_string());
    }
    Ok(pid)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-V") {
        let version = colorful_version!();
        version.print_and_exit();
    }
    let prog = &args[0];

    let mut verbose = false;
    let mut pid_arg: Option<&str> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-v" | "--verbose" => verbose = true,
            "--version" => version(),
            "-h" | "--help" => usage(prog),
            other if !other.starts_with('-') && pid_arg.is_none() => pid_arg = Some(other),
            other => {
                eprintln!("Error: unknown option '{}'", other);
                usage(prog);
            }
        }
        i += 1;
    }

    let pid_str = pid_arg.unwrap_or_else(|| {
        eprintln!("Error: missing PID argument");
        usage(prog);
    });

    let pid = parse_pid(pid_str).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });

    let config = Config::new(pid).with_verbose(verbose);

    if verbose {
        eprintln!("[START] PID={}, retries={}, delay={}ms", pid, config.retries, config.retry_delay.as_millis());
    }

    let mut activator = WindowActivator::new(config);

    match activator.execute() {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
