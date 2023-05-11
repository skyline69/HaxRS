#!/bin/bash

# Check if Rust and Cargo are installed
if ! command -v rustc &> /dev/null || ! command -v cargo &> /dev/null; then
    echo -e "\e[31mError: Rust and Cargo are required but not installed.\e[0m"
    echo -e "Please install Rust and Cargo using rustup: \e[94mhttps://rustup.rs/\e[0m"
    exit 1
fi

# Check if nmap is installed
if ! command -v nmap &> /dev/null; then
    echo -e "\e[31mError: nmap is required but not installed.\e[0m"
    echo -e "Please follow the instructions here ( \e[94mhttps://github.com/skyline69/HaxRS#how-to-install-nmap-on-linux\e[0m ) to install nmap on your system."
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

# after installation is done, remove the repository directory
cd .. && rm -rf HaxRS

echo -e "\e[32mHaxRS has been installed successfully. You can run it with the command 'haxrs'.\e[0m"