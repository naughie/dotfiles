if status is-interactive
    # Commands to run in interactive sessions can go here
end

fish_add_path $HOME/bin

set -x LC_ALL en_US.UTF-8
set -x LANG en_US.UTF-8

set -x CPATH /usr/local/include $CPATH
set -x LIBRARY_PATH /usr/local/lib $LIBRARY_PATH
set -x LD_LIBRARY_PATH /usr/local/lib $LD_LIBRARY_PATH

# set EDITOR to either nvim or vim
command -v nvim >/dev/null &&
  set -x EDITOR nvim ||
  set -x EDITOR vim

# set config env
set -x XDG_CONFIG_HOME $HOME/.config
