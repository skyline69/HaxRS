#!/bin/bash

# Check if Rust and Cargo are installed
if ! command -v rustc &> /dev/null || ! command -v cargo &> /dev/null; then
    echo "Error: Rust and Cargo are required but not installed."
    echo "Please install Rust and Cargo using rustup: https://rustup.rs/"
    exit 1
fi

# Check if nmap is installed
if ! command -v nmap &> /dev/null; then
    echo "Error: nmap is required but not installed."
    echo "Please follow the instructions in README.md to install nmap on your system."
    exit 1
fi

# Clone the HaxRS repository
git clone https://github.com/skyline69/HaxRS.git

# Change to the repository directory
cd HaxRS || exit

# Build the application
cargo build --release

# Copy the binary to /usr/local/bin
sudo cp target/release/haxrs /usr/local/bin/

echo "HaxRS has been installed successfully. You can run it with the command 'haxrs'."
