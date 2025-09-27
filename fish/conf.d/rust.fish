# enable rust
set -x CARGO_HOME $HOME/etc/cargo
set -x RUSTUP_HOME $HOME/etc/rustup
fish_add_path $CARGO_HOME/bin
