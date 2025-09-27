# init rbenv
set -x RBENV_ROOT $HOME/etc/rbenv
fish_add_path $RBENV_ROOT/bin
command -v rbenv >/dev/null && eval "$(rbenv init -)"
