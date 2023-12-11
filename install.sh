#!/bin/bash

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

if ! command_exists rustc; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    echo "Rust installed successfully."
else
    echo "Rust is already installed."
fi

if command_exists pkg-config && pkg-config --exists gtk+-3.0; then
  echo "GTK3 is already installed."
  exit 0
fi


echo "GTK3 is not installed. Installing GTK3..."

# Install GTK3 on macOS using Homebrew
if command_exists brew; then
    brew install gtk+3
else
    # Install GTK3 on Linux
    if command_exists apt-get; then
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev
    elif command_exists yum; then
        sudo yum install -y gtk3-devel
    else
        echo "Unsupported package manager. Please install GTK3 manually."
        exit 1
    fi
fi

echo "GTK3 installed successfully."
