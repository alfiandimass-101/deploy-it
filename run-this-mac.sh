#!/bin/bash
echo "--- Script starting, running as user: $(whoami) ---"

# Jalankan rustup dengan menunjuk ke instalasi milik 'runner'.
# Ini diperlukan karena skrip ini dijalankan oleh root.
export RUSTUP_HOME="/Users/runner/.rustup"
export CARGO_HOME="/Users/runner/.cargo"
echo "Setting default Rust toolchain to nightly..."
/Users/runner/.cargo/bin/rustup default nightly

echo "Configuring root's shell environment (.zshrc and .bashrc)..."

# Mengkonfigurasi file shell untuk root.
# Menggunakan 'append' (>>) agar tidak menimpa file yang ada.
echo 'export TERM=xterm-256color' >> /var/root/.zshrc
echo 'export TERM=xterm-256color' >> /var/root/.bashrc

# Menggunakan variabel GITHUB_WORKSPACE untuk path yang andal.
echo "alias workdir='cd /Users/runner/work/deploy-it/deploy-it'" >> /var/root/.zshrc
echo "alias workdir='cd /Users/runner/work/deploy-it/deploy-it'" >> /var/root/.bashrc

# Menambahkan path cargo milik 'runner' ke PATH milik root.
# Tanda kutip tunggal penting untuk mencegah perluasan $PATH secara prematur.
echo 'export PATH="/Users/runner/.cargo/bin:$PATH"' >> /var/root/.zshrc
echo 'export PATH="/Users/runner/.cargo/bin:$PATH"' >> /var/root/.bashrc

echo "--- Configuration finished. ---"

# Tambahkan perintah lain yang memerlukan root di sini
