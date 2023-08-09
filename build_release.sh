echo "Updating rust targets"
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
echo "Rust targets updated"
echo "=========================="
echo "Building for Windows"
cargo build --release --target x86_64-pc-windows-gnu
echo "Done!"
echo "=========================="
echo "Building for Linux"
cargo build --release --target x86_64-unknown-linux-gnu
echo "Done!"