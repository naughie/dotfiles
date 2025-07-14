fish_add_path /usr/local/bin
fish_add_path $HOME/bin

set -x ETC_LOCAL $HOME/etc

# set terminal color
# set -x TERM wezterm
set -x TERM alacritty

set -x LC_ALL en_US.UTF-8
set -x LANG en_US.UTF-8

set -x CPATH /usr/local/include $CPATH
set -x LIBRARY_PATH /usr/local/lib $LIBRARY_PATH
set -x LD_LIBRARY_PATH /usr/local/lib $LD_LIBRARY_PATH

set -x CLICOLOR true
set -x KEYTIMEOUT 1

set -x LS_COLORS 'tw=30;47:ow=30;47:'

# set EDITOR to either nvim or vim
command -v nvim >/dev/null &&
  set -x EDITOR nvim ||
  set -x EDITOR vim

# set config env
set -x XDG_CONFIG_HOME $HOME/.config

# init anaconda (python)
set -x CONDA_HOME $ETC_LOCAL/anaconda3
test -f $CONDA_HOME/bin/conda && eval "$($CONDA_HOME/bin/conda shell.fish hook)"
# fish_add_path $CONDA_HOME/bin

# init rbenv
set -x RBENV_ROOT $ETC_LOCAL/rbenv
fish_add_path $RBENV_ROOT/bin
command -v rbenv >/dev/null && eval "$(rbenv init -)"

# nvm (node version manager)
# set -x NVM_DIR $HOME/etc/nvm
# [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# export PATH=$ETC_LOCAL/nodejs/bin:$PATH

set FNM_PATH $ETC_LOCAL/fnm/bin
set -x FNM_DIR $ETC_LOCAL/fnm
if [ -d "$FNM_PATH" ]
  set PATH "$FNM_PATH" $PATH
  fnm env | source
end

set -x DENO_INSTALL $ETC_LOCAL/deno
set -x DENO_INSTALL_ROOT $DENO_INSTALL
fish_add_path $DENO_INSTALL/bin

set -x BUN_INSTALL $ETC_LOCAL/bun
fish_add_path $BUN_INSTALL/bin

# enable rust
set -x CARGO_HOME $ETC_LOCAL/cargo
set -x RUSTUP_HOME $ETC_LOCAL/rustup
fish_add_path $CARGO_HOME/bin

# tex
set TEX_YEAR 2024
set TEX_DIST x86_64-linux
# set TEX_DIST universal-darwin
# set -x MANPATH "/usr/local/texlive/$TEX_YEAR/texmf-dist/doc/man:$MANPATH"
# set -x INFOPATH /usr/local/texlive/$TEX_YEAR/texmf-dist/doc/info $INFOPATH
fish_add_path /usr/local/texlive/$TEX_YEAR/bin/$TEX_DIST
set -x TEXMFLOCAL /usr/local/texlive/texmf-local
set -x TEXMFHOME $HOME/etc/texmf

# Go
set -x GOPATH $ETC_LOCAL/go
fish_add_path $GOPATH/bin
