export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

export XDG_CONFIG_HOME=$HOME/.config
export XDG_BIN_HOME=$HOME/bin

export PATH=$HOME/bin:$PATH

# Go
export GOROOT=$HOME/etc/go
export GOPATH=$GOROOT/vendor
export PATH=${GOPATH}/bin:${GOROOT}/bin:$PATH

# Node
export FNM_DIR=$HOME/etc/fnm
export PATH=$FNM_DIR/bin:$PATH

# Deno
export DENO_INSTALL=$HOME/etc/deno
export DENO_INSTALL_ROOT=$DENO_INSTALL
export PATH=$DENO_INSTALL/bin:$PATH

# Bun
export BUN_INSTALL=$HOME/etc/bun
export PATH=$BUN_INSTALL/bin:$PATH

# Python


# Ruby
export RBENV_ROOT=$HOME/etc/rbenv
export PATH=$RBENV_ROOT/bin:$PATH

# Rust
export CARGO_HOME=$HOME/etc/cargo
export RUSTUP_HOME=$HOME/etc/rustup
export PATH=$CARGO_HOME/bin:$PATH

# TeX
export TEXMFLOCAL=/usr/local/texlive/texmf-local
export TEXMFHOME=$HOME/etc/texmf
TEX_YEAR=2025
case "$(uname -ms)" in
    "Linux x86_64")
        TEX_DIST=x86_64-linux
        ;;
    "Darwin arm64")
        TEX_DIST=universal-darwin
        ;;
esac
export PATH=/usr/local/texlive/$TEX_YEAR/bin/$TEX_DIST:$PATH

# Flutter
export PATH=$HOME/etc/flutter/bin:$PATH


case "$-" in
    *i*)
        exec fish -il
        ;;
esac
