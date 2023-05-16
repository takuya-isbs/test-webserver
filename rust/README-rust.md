## install

example for Ubuntu

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo version
cd test-webserver/rust
apt install gcc
cargo test

sh install.sh
```

## uninstall

```bash
sh uninstall.sh
```
