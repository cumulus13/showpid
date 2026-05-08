# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-29

### Added
- Initial Rust implementation of showpid
- Window enumeration by PID
- Foreground window activation
- Window restoration from minimized state
- Retry logic with configurable attempts
- Thread input attachment for reliable activation
- Verbose mode for debugging
- Comprehensive error handling
- Unit tests
- CI/CD workflow
- GitHub Actions integration

### Changed
- Converted from Python to Rust for better performance
- Added proper error messages and exit codes
- Improved documentation

### Migration from Python
- Python: `showpid.py <PID>`
- Rust: `showpid <PID>`
- Added `-v`/`--verbose` flag
- Added `-h`/`--help` flag