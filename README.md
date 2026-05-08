# showpid

[![Crates.io](https://img.shields.io/crates/v/showpid.svg)](https://crates.io/crates/showpid)
[![Documentation](https://docs.rs/showpid/badge.svg)](https://docs.rs/showpid)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/cumulus13/showpid/workflows/CI/badge.svg)](https://github.com/cumulus13/showpid/actions)

Bring a window of a specified process ID (PID) to the foreground on Windows.

## Features

- 🚀 **Fast & Lightweight** - Pure Rust implementation with minimal dependencies
- 🔄 **Retry Logic** - Configurable retry mechanism for transient failures
- 🪟 **Multiple Windows** - Handles processes with multiple windows
- 🔧 **Thread-Safe** - Proper thread input attachment for reliable foreground activation
- 📊 **Verbose Mode** - Detailed logging for troubleshooting
- 🛡️ **Error Handling** - Comprehensive error handling with meaningful messages
- ⚡ **Zero-Cost Abstractions** - Safe Rust wrapper over Windows API

## Installation

### From Crates.io

```bash
cargo install showpid
```

### From Source

```bash
git clone https://github.com/cumulus13/showpid.git
cd showpid
cargo install --path .
```

### Pre-built Binaries

Download the latest release from the [Releases](https://github.com/cumulus13/showpid/releases) page.

## Usage

### Basic Usage

```bash
# Bring window of PID 1234 to foreground
showpid 1234

# Verbose mode
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
```

### Examples

```bash
# Find a process PID first (using tasklist)
tasklist | findstr notepad.exe

# Bring notepad to foreground (assuming PID is 1234)
showpid 1234

# With verbose output for debugging
showpid -v 1234
```

### Exit Codes

- `0` - Success: Window brought to foreground
- `1` - Error: No window found or operation failed

## How It Works

1. Enumerates all visible windows on the system
2. Filters windows by the specified process ID
3. Restores the window if minimized
4. Attaches thread input for reliable foreground activation
5. Sets the window as the foreground window
6. Implements retry logic for transient failures

## Requirements

- Windows 10 or later (uses Windows API)
- 64-bit or 32-bit Windows

## Development

### Prerequisites

- Rust 1.70 or later
- Windows SDK

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check for warnings
cargo clippy -- -D warnings
```

### Project Structure

```
showpid/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Package configuration
├── Cargo.lock           # Dependency lock file
├── README.md            # Project documentation
├── LICENSE              # MIT License
├── .gitignore           # Git ignore rules
├── .github/
│   └── workflows/
│       ├── ci.yml       # CI workflow
│       └── publish.yml  # Crates.io publish workflow
└── CHANGELOG.md         # Version history
```

## API Documentation

While `showpid` is primarily a CLI tool, it can also be used as a library:

```rust
use showpid::WindowActivator;
use showpid::Config;

let config = Config::new(1234)
    .with_verbose(true);
let mut activator = WindowActivator::new(config);
activator.execute().expect("Failed to activate window");
```

Full API documentation is available on [docs.rs](https://docs.rs/showpid).

## Comparison with Original Python Version

| Feature | Python (showpid.py) | Rust (showpid) |
|---------|-------------------|----------------|
| Speed | Moderate | Fast (compiled) |
| Binary Size | ~0 KB + Python | ~2 MB standalone |
| Dependencies | win32gui, win32process | windows crate |
| Error Handling | Basic | Comprehensive |
| Retry Logic | ❌ | ✅ (configurable) |
| Multiple Windows | First only | All windows tried |
| Verbose Mode | ❌ | ✅ |
| Thread Safety | N/A | ✅ |
| Testing | ❌ | ✅ |
| Cross-compilation | N/A | ✅ |

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author
        
**Hadi Cahyadi** - [cumulus13@gmail.com](mailto:cumulus13@gmail.com)
    

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)
 
[Support me on Patreon](https://www.patreon.com/cumulus13)

## Acknowledgments

- Original Python version by Hadi Cahyadi
- Windows API bindings provided by [windows-rs](https://github.com/microsoft/windows-rs)

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/cumulus13/showpid/issues) on GitHub.

