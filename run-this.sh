#!/bin/bash
echo "--- Script starting, running as user: $(whoami) ---"

echo "Setting default Rust toolchain to nightly..."
apt install rustup
rustup default nightly

echo "Configuring root's shell environment (.bashrc)..."
echo 'export TERM=xterm-256color' >> /root/.bashrc

# Use the GITHUB_WORKSPACE variable for a reliable path.
echo "alias workdir='cd /home/runner/work/deploy-it/deploy-it'" >> /root/.bashrc

echo "--- Configuration finished. ---"

/home/runner/work/deploy-it/deploy-it/Projects/ep/ep
