rustup default nightly
sudo echo "export TERM=xterm-256color" > /var/root/.zshrc
sudo echo "export TERM=xterm-256color" > /var/root/.bashrc
sudo echo "alias workdir='cd /Users/runner/work/deploy-it/deploy-it'" > /var/root/.zshrc
sudo echo "alias workdir='cd /Users/runner/work/deploy-it/deploy-it'" > /var/root/.bashrc
sudo echo 'export PATH="/Users/runner/.cargo/bin:$PATH"' > /var/root/.zshrc
sudo echo 'export PATH="/Users/runner/.cargo/bin:$PATH"' > /var/root/.bashrc
