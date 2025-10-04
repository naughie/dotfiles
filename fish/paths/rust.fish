function setup_rust
    if [ -d "$CARGO_HOME" ]
        fish_add_path "$CARGO_HOME/bin"
    end
end
set -ax setup_list rust
