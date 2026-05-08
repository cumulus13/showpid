# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-29

### Added
- Initial cross-platform release
- Windows support via Win32 API (EnumWindows, SetForegroundWindow)
- Linux support via X11 (XRaiseWindow, XSetInputFocus)
- macOS support via AppleScript (osascript)
- Retry logic with configurable attempts and delay
- Verbose mode for detailed progress output
- Comprehensive error handling with helpful messages
- Proper exit codes (0 = success, 1 = error)
- Unit tests for argument parsing
- CI/CD pipeline with GitHub Actions
- Cross-platform release artifacts
- Automatic publishing to crates.io
- Security auditing via cargo-audit