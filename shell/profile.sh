export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

_generated_profile_sh="$HOME/etc/dotfiles/shell/generated.profile.sh"
test -f "${_generated_profile_sh}" && . "${_generated_profile_sh}"

export PATH="${XDG_BIN_HOME}:$PATH"

# Go
export PATH="${GOPATH}/bin:${GOROOT}/bin:$PATH"

# Node
export PATH="${FNM_DIR}/bin:$PATH"
if [ -x "${FNM_DIR}/bin/fnm" ]; then
    eval "$(fnm env --shell bash)"
fi

# Deno
export PATH="${DENO_INSTALL_ROOT}:$PATH"

# Bun
export PATH="${BUN_INSTALL}/bin:$PATH"

# Python

export PATH="${UV_INSTALL_DIR}:${UV_PYTHON_BIN_DIR}:${UV_TOOL_BIN_DIR}:$PATH"
export HF_HOME="$HOME/etc/hf"

# Ruby
export PATH="${RBENV_ROOT}/bin:$PATH"

# Rust
export PATH="${CARGO_HOME}/bin:$PATH"

# TeX
export TEXMFLOCAL=/usr/local/texlive/texmf-local
export TEXMFHOME="$HOME/etc/texmf"
_tex_year=2025
case "$(uname -ms)" in
    "Linux x86_64")
        _tex_dist=x86_64-linux
        ;;
    "Darwin arm64")
        _tex_dist=universal-darwin
        ;;
esac
export PATH="/usr/local/texlive/${_tex_year}/bin/${_tex_dist}:$PATH"

# Flutter
export PATH="$HOME/etc/flutter/bin:$PATH"


case "$-" in
    *i*)
        test -x "${_FISH_INSTALL}/fish" && exec "${_FISH_INSTALL}/fish" -il
        ;;
esac
