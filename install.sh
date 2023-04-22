#!/bin/bash

# This script will install the lazycli program

# Check if Rust is installed
if ! [ -x "$(command -v rustc)" ]; then
  echo 'Error: Rust is not installed.' >&2
  exit 1
fi

# Download the program
echo 'Downloading program...'
curl -L https://github.com/rijulrr/lazycli/archive/main.zip -o lazycli.zip

# Unzip the program
echo 'Unzipping program...'
unzip lazycli.zip

# Change into the program directory
echo 'Changing into program directory...'
cd lazycli-main

# Build the program
echo 'Building program...'
cargo build --release

# Install the program
echo 'Installing program...'
cargo install --path .

# Clean up
echo 'Cleaning up...'
cd ..
rm -rf lazycli-main lazycli.zip

echo 'Installation complete!'
