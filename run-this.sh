rustup default nightly
sudo su -l root -c echo "export TERM=xterm-256color" >> /var/root/.zshrc
sudo su -l root -c echo "alias workdir='cd /Users/runner/work/deploy-it/deploy-it'" >> /var/root/.bashrc
sudo su -l root -c 'export PATH="/Users/runner/.cargo/bin:$PATH"' > /var/root/.zshrc
