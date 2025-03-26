#!/bin/bash

echo "Building the Rust project..."
echo "          "
echo "          "
echo "          "


echo "Removing existing directories, if there is any..."
# Remove existing builds directory and executable if they exist
rm -rf builds gaspy

echo "Building..."
echo "          "
echo "          "   

# Create fresh builds directory
mkdir -p builds

# Compile the Rust project and place the output in the builds directory
cargo build --release --target-dir builds

echo "Cleaning up..."
echo "          "
echo "          "
# Copy the compiled binary to the root of the project
cp builds/release/gaspy .

rm -rf builds

echo "Done! The executable is in the root of the project :)"
echo "          "
echo "          "