#!/bin/bash
echo "--- Script starting, running as user: $(whoami) ---"

# Run rustup pointing to the 'runner' user's installation.
# This is necessary because this script is run by root.
export RUSTUP_HOME="/home/runner/.rustup"
export CARGO_HOME="/home/runner/.cargo"
echo "Setting default Rust toolchain to nightly..."
/home/runner/.cargo/bin/rustup default nightly

echo "Configuring root's shell environment (.bashrc)..."

# Configure the shell file for root.
# Use 'append' (>>) to avoid overwriting the existing file.
echo 'export TERM=xterm-256color' >> /root/.bashrc

# Use the GITHUB_WORKSPACE variable for a reliable path.
echo "alias workdir='cd /home/runner/work/deploy-it/deploy-it'" >> /root/.bashrc

# Add the 'runner' user's cargo bin path to the root's PATH.
# Single quotes are important to prevent premature expansion of $PATH.
echo 'export PATH="/home/runner/.cargo/bin:$PATH"' >> /root/.bashrc

echo "--- Configuration finished. ---"

# Add any other commands that require root here
