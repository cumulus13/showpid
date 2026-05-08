# PowerShell build script
Write-Host "Building showpid for all platforms..."

# Add targets
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin

# Build for each platform
Write-Host "Building for Windows..."
cargo build --release --target x86_64-pc-windows-msvc

Write-Host "Building for Linux... (cross-compilation may fail)"
cargo build --release --target x86_64-unknown-linux-gnu

Write-Host "Building for macOS... (cross-compilation may fail)"
cargo build --release --target x86_64-apple-darwin

Write-Host "Build complete! Check target/ directory for binaries."
