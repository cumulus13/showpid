# showpid

[![Crates.io](https://img.shields.io/crates/v/showpid.svg)](https://crates.io/crates/showpid)
[![Documentation](https://docs.rs/showpid/badge.svg)](https://docs.rs/showpid)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/cumulus13/showpid/workflows/CI/badge.svg)](https://github.com/cumulus13/showpid/actions)
[![Platform Support](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)]()

Bring a window of a specified process ID (PID) to the foreground. Cross-platform support for Windows, Linux, and macOS.

## Features

- 🚀 **Fast & Lightweight** - Pure Rust implementation with minimal dependencies
- 🌍 **Cross-Platform** - Supports Windows, Linux (X11), and macOS
- 🔄 **Retry Logic** - Configurable retry mechanism for transient failures
- 🪟 **Multiple Windows** - Handles processes with multiple windows
- 🔧 **Platform-Native** - Uses native APIs for optimal performance on each platform
- 📊 **Verbose Mode** - Detailed logging for troubleshooting
- 🛡️ **Error Handling** - Comprehensive error handling with meaningful messages
- ⚡ **Zero-Cost Abstractions** - Safe Rust wrapper over platform-specific APIs
- 📦 **Standalone Binary** - Single executable, no runtime dependencies required
- 🧪 **Well Tested** - Unit tests and cross-platform CI testing

## Platform Support

| Platform | API Used | Minimum Version | Notes |
|----------|----------|-----------------|--------|
| Windows  | Windows API (Win32) | Windows 10+ | Native Win32 API through `windows-rs` |
| Linux    | X11 (Xlib) | X11 compatible | Requires `libx11-dev` on some distributions |
| macOS    | CoreGraphics + AppleScript | macOS 10.12+ | Uses `osascript` for reliable window activation |

## Installation

### From Crates.io

```bash
cargo install showpid
```

### From Source

```bash
git clone https://github.com/cumulus13/showpid.git
cd showpid

# Linux: Install dependencies first
sudo apt-get install libx11-dev  # Ubuntu/Debian
# or
sudo dnf install libX11-devel     # Fedora/RHEL

# Build and install
cargo install --path .
```

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/cumulus13/showpid/releases) page:

- **Windows**: `showpid-windows-x86_64.zip`
- **Linux**: `showpid-linux-x86_64.tar.gz`
- **macOS**: `showpid-macos-x86_64.tar.gz`

### Platform-Specific Requirements

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install libx11-dev

# Fedora/RHEL
sudo dnf install libX11-devel

# Arch Linux
sudo pacman -S libx11
```

#### macOS
No additional dependencies required. Uses built-in CoreGraphics and AppleScript frameworks.

#### Windows
No additional dependencies required. Uses built-in Windows API.

## Usage

### Basic Usage

```bash
# Bring window of PID 1234 to foreground
showpid 1234

# Verbose mode with detailed output
showpid -v 1234
showpid --verbose 1234

# Show help
showpid -h
showpid --help
```

### Command Line Options

```
Usage: showpid [OPTIONS] <PID>

Brings the window of the specified PID to the foreground.

Arguments:
  <PID>         Process ID to bring to foreground

Options:
  -v, --verbose  Enable verbose output
  -h, --help     Show this help message
  -V, --version  Print version information
```

### Platform-Specific Examples

#### Windows
```bash
# Find notepad PID
tasklist | findstr notepad.exe

# Bring to foreground
showpid 1234

# With verbose output
showpid -v 1234
```

#### Linux
```bash
# Find Firefox PID
pgrep firefox

# Bring to foreground
showpid 1234

# With verbose output
showpid -v 1234
```

#### macOS
```bash
# Find Safari PID
pgrep Safari

# Bring to foreground
showpid 1234

# With verbose output
showpid -v 1234
```

### Using as a Library

```rust
use showpid::{Config, WindowActivator};

fn main() {
    let config = Config::new(1234)
        .with_verbose(true);
    
    let mut activator = WindowActivator::new(config);
    
    match activator.execute() {
        Ok(()) => println!("Window activated successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## How It Works

### Windows
1. Enumerates all visible windows using `EnumWindows`
2. Filters windows by process ID using `GetWindowThreadProcessId`
3. Restores window if minimized with `ShowWindow`
4. Attaches thread input for reliable foreground activation
5. Sets window as foreground with `SetForegroundWindow`

### Linux (X11)
1. Connects to X display and queries all windows
2. Reads `_NET_WM_PID` property to match process IDs
3. Raises window with `XRaiseWindow`
4. Sets input focus with `XSetInputFocus`
5. Maps window if unmapped with `XMapWindow`

### macOS
1. Uses `CGWindowListCopyWindowInfo` to enumerate windows
2. Filters by owner PID from window properties
3. Uses AppleScript to bring process to foreground
4. Handles permissions and accessibility requirements

## Exit Codes

- `0` - Success: Window brought to foreground
- `1` - Error: No window found, operation failed, or invalid input

## Environment Variables

- `RUST_LOG` - Set logging level (e.g., `RUST_LOG=debug showpid 1234`)
  - Levels: `error`, `warn`, `info`, `debug`, `trace`

## Troubleshooting

### Linux
- **Error: "Failed to open X display"**: Make sure you're running in an X11 session (not Wayland)
- **No windows found**: Ensure the process has visible windows and you have proper permissions

### macOS
- **Permission denied**: Grant accessibility permissions in System Preferences > Security & Privacy > Privacy > Accessibility
- **osascript errors**: Ensure you have AppleScript permissions enabled

### Windows
- **Access denied**: Run as administrator if targeting system processes
- **No windows found**: Some processes may not have visible windows

## Development

### Prerequisites

- Rust 1.70 or later
- Platform-specific dependencies (see Installation section)

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run clippy lints
cargo clippy -- -D warnings

# Format code
cargo fmt --all -- --check
```

### Cross-Compilation

```bash
# Cross-compile for Windows from Linux
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Cross-compile for macOS from Linux
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

### Project Structure

```
showpid/
├── src/
│   └── main.rs          # Main application with platform modules
│   ├── platform/
│   │   ├── windows.rs   # Windows-specific implementation
│   │   ├── linux.rs     # Linux/X11-specific implementation
│   │   └── macos.rs     # macOS-specific implementation
├── Cargo.toml           # Package configuration
├── Cargo.lock           # Dependency lock file
├── README.md            # Project documentation
├── LICENSE              # MIT License
├── CHANGELOG.md         # Version history
├── .gitignore           # Git ignore rules
└── .github/
    └── workflows/
        ├── ci.yml       # Multi-platform CI workflow
        └── publish.yml  # Multi-platform publish workflow
```

## Comparison with Original Python Version

| Feature | Python (showpid.py) | Rust (showpid) |
|---------|-------------------|----------------|
| Speed | Moderate | Fast (compiled) |
| Binary Size | ~0 KB + Python | ~2-5 MB standalone |
| Dependencies | win32gui, win32process | Platform-native APIs |
| Platform Support | Windows only | Windows, Linux, macOS |
| Error Handling | Basic | Comprehensive |
| Retry Logic | ❌ | ✅ (configurable) |
| Multiple Windows | First only | All windows tried |
| Verbose Mode | ❌ | ✅ |
| Logging | Print statements | Structured logging (log/env_logger) |
| Thread Safety | N/A | ✅ |
| Testing | ❌ | ✅ (unit tests + CI) |
| Cross-compilation | N/A | ✅ |
| CI/CD | N/A | ✅ (GitHub Actions) |
| Package Registry | N/A | ✅ (crates.io) |

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## API Documentation

Full API documentation is available on [docs.rs](https://docs.rs/showpid).

### Main API Types

- `Config` - Configuration for window activation
- `WindowActivator` - Main window finder and activator
- `WindowInfo` - Platform-specific window information

### Example: Programmatic Usage

```rust
use showpid::{Config, WindowActivator};

// Create configuration
let config = Config::new(1234)
    .with_verbose(true)
    .with_retries(5)
    .with_retry_delay(std::time::Duration::from_millis(200));

// Create activator
let mut activator = WindowActivator::new(config);

// Execute with custom error handling
match activator.execute() {
    Ok(()) => {
        println!("Window activated successfully");
        std::process::exit(0);
    }
    Err(e) => {
        eprintln!("Failed to activate window: {}", e);
        std::process::exit(1);
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Ensure cross-platform compatibility
4. Add tests for new features
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Adding Support for New Platforms

To add support for a new platform:

1. Create a new conditionally compiled module in `src/main.rs`
2. Implement `WindowActivator` for the new platform
3. Add platform-specific dependencies to `Cargo.toml`
4. Update CI workflow to test the new platform
5. Update this README with new platform requirements

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author
        
**Hadi Cahyadi** - [cumulus13@gmail.com](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

- GitHub: [@cumulus13](https://github.com/cumulus13)

## Acknowledgments

- Original Python version by Hadi Cahyadi
- Windows API bindings provided by [windows-rs](https://github.com/microsoft/windows-rs)
- X11 bindings provided by the Rust community
- CoreGraphics bindings for macOS support
- All contributors and users of this project

## Support

If you encounter any issues or have questions:

1. Check the [Troubleshooting](#troubleshooting) section
2. Search [existing issues](https://github.com/cumulus13/showpid/issues)
3. [Open a new issue](https://github.com/cumulus13/showpid/issues/new) with:
   - Your operating system and version
   - Steps to reproduce
   - Verbose output (`showpid -v <PID>`)
   - Expected vs actual behavior

## Roadmap

- [ ] Wayland support for Linux
- [ ] Apple Silicon (ARM64) native builds
- [ ] Window selection by title pattern matching
- [ ] Multiple window selection interface
- [ ] Configuration file support
- [ ] Shell completion scripts
- [ ] System tray integration
- [ ] Web interface for remote control
