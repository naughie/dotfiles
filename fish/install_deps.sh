#!/bin/bash

set -e
set -o pipefail

install_anaconda() {
    CONDA_I=${CONDA_HOME:-$HOME/etc/anaconda3}
    test -x $CONDA_I/bin/conda && return 0

    echo 'Install Anaconda3'
    CONDA_LATEST=2025.06-0
    curl -fsSL https://repo.anaconda.com/archive/Anaconda3-${CONDA_LATEST}-Linux-x86_64.sh | bash -s -- -b -p $CONDA_I

    echo "Installed Anaconda3 to $CONDA_I"
}

install_bun() {
    BUN_I=${BUN_INSTALL:-$HOME/etc/bun}
    test -x $BUN_I/bin/bun && return 0

    echo 'Install Bun'
    curl -fsSL https://bun.com/install | bash
    echo "Installed Bun to $BUN_I"
    echo 'Remove configs from $HOME/.config/fish/config.fish'
}

install_deno() {
    DENO_I=${DENO_INSTALL:-$HOME/etc/deno}
    test -x $DENO_I/bin/deno && return 0

    echo 'Install Deno'
    curl -fsSL https://deno.land/install.sh | sh
    echo "Installed Deno to $DENO_I"
}

install_fish() {
    test -x /usr/bin/fish && return 0

    echo 'Install fish'
    FISH_LATEST=$(curl -s -f https://api.github.com/repos/fish-shell/fish-shell/releases/latest | jq -r .tag_name)
    curl -sfL "https://github.com/fish-shell/fish-shell/releases/download/$FISH_LATEST/fish-${FISH_LATEST}-linux-x86_64.tar.xz" | tar xJ -C $HOME/bin
    echo "Installed fish to $HOME/bin/fish"
    echo 'Run:'
    echo ''
    echo 'sudo mv $HOME/bin/fish /usr/bin/fish && sudo chown root:root /usr/bin/fish'
    echo 'echo /usr/bin/fish | sudo tee -a /etc/shells'
    echo 'chsh -s /usr/bin/fish'
}

install_go() {
    GO_I=${GOPATH:-$HOME/etc/go}
    test -x $GO_I/bin/go && return 0

    echo 'Install Go'
    GO_LATEST=$(curl -s "https://go.dev/dl/?mode=json" | jq -r '.[0].version')
    curl -sfL https://go.dev/dl/${GO_LATEST}.linux-amd64.tar.gz | tar xz -C "$(dirname $GO_I)"
    echo "Installed Go to $GO_I"
}

install_neovim() {
    test -x $HOME/bin/nvim && return 0

    echo 'Install neovim'
    NVIM_LATEST=$(curl -s -f https://api.github.com/repos/neovim/neovim/releases/latest | jq -r .tag_name)
    curl -sL https://github.com/neovim/neovim/releases/latest/download/nvim-linux-x86_64.appimage -o $HOME/bin/nvim
    chmod u+x $HOME/bin/nvim
    echo "Installed neovim to $HOME/bin/nvim"
    echo 'Run if libfuse2 is not installed:'
    echo ''
    echo 'cd $HOME/bin'
    echo './nvim --appimage-extract'
    echo 'mv squashfs-root nvim-squashfs-root'
    echo 'rm nvim && ln -s ./nvim-squashfs-root/AppRun nvim'
    echo './nvim --version'
}

install_node() {
    FNM_I=${FNM_DIR:-$HOME/etc/fnm}
    test -x $FNM_I/bin/fnm && return 0

    echo 'Install fnm'
    curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir $FNM_I --skip-shell
    echo "Installed fnm to $FNM_I"
    echo 'Run:'
    echo ''
    echo 'fnm list-remote --lts'
    echo 'fnm list-remote --latest'
    echo ''
    echo 'fnm install --lts'
    echo 'fnm default lts-latest'
    echo 'fnm use lts-latest'
    echo ''
    echo 'fnm install --latest'
    echo 'fnm default latest'
    echo 'fnm use latest'
}

install_ruby() {
    RBENV_I=${RBENV_ROOT:-$HOME/etc/rbenv}
    test -x $RBENV_I/bin/rbenv && return 0

    echo 'Install rbenv'
    git clone https://github.com/rbenv/rbenv.git $RBENV_I
    git clone https://github.com/rbenv/ruby-build.git $RBENV_I/plugins/ruby-build

    echo "Installed rbenv to $RBENV_I"
    echo 'Install dependencies:'
    echo ''
    echo 'sudo apt-get install -y build-essential autoconf libssl-dev libyaml-dev zlib1g-dev libffi-dev libgmp-dev rustc'
    echo ''
    echo 'Run:'
    echo ''
    echo 'rbenv install -l'
    echo 'rbenv install -L'
    echo 'rbenv global VERSION'
}

install_rust() {
    CARGO_I=${CARGO_HOME:-$HOME/etc/cargo}
    test -x $CARGO_I/bin/cargo && return 0

    echo 'Install Cargo'
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    echo "Installed Cargo to $CARGO_I"
}

install_starship() {
    test -x $HOME/bin/starship && return 0

    echo 'Install Starship'
    curl -sS https://starship.rs/install.sh | sh -s -- -y -b $HOME/bin
    echo "Installed Starship to $HOME/bin/starship"
}

print_help() {
  echo "Usage: $0 [tool1] [tool2] ..."
  echo ""
  echo "A script to install various development tools."
  echo "Make sure you have installed jq, curl, git."
  echo ""
  echo "Available Tools (aliases in parentheses):"
  echo "  - all"
  echo ""
  echo "  - anaconda (anaconda3)"
  echo "  - bun"
  echo "  - deno"
  echo "  - fish"
  echo "  - go (golang)"
  echo "  - neovim (nvim)"
  echo "  - node (fnm)"
  echo "  - ruby (rb)"
  echo "  - rust (cargo, rustc)"
  echo "  - starship"
  echo ""
  echo "Example: $0 nvim bun anaconda3"
}

command -v jq >/dev/null
command -v curl >/dev/null
command -v git >/dev/null
test -d $HOME/bin || mkdir $HOME/bin

if [ "$#" -eq 0 ]; then
  print_help
  exit 1
fi

if [ "$1" = "all" ]; then
  all_installers=$(declare -F | awk '{print $3}' | grep '^install_')

  for installer in $all_installers; do
    "$installer"
  done

  exit 0
fi

for arg in "$@"; do
  target_tool="$arg"

  case "$arg" in
    anaconda3)
      target_tool="anaconda"
      ;;
    golang)
      target_tool="go"
      ;;
    nvim)
      target_tool="neovim"
      ;;
    fnm)
      target_tool="node"
      ;;
    rb)
      target_tool="ruby"
      ;;
    cargo|rustc)
      target_tool="rust"
      ;;
  esac

  install_function="install_${target_tool}"

  if [ "$(type -t "$install_function")" = "function" ]; then
    "$install_function"
  else
    echo "Error: Unknown tool '$arg'. No installer function"
  fi
done
