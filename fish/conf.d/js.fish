# nvm (node version manager)
# set -x NVM_DIR $HOME/etc/nvm
# [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

set FNM_PATH $HOME/etc/fnm/bin
set -x FNM_DIR $HOME/etc/fnm
if [ -d "$FNM_PATH" ]
  set PATH "$FNM_PATH" $PATH
  fnm env | source
end

set -x DENO_INSTALL $HOME/etc/deno
set -x DENO_INSTALL_ROOT $DENO_INSTALL
fish_add_path $DENO_INSTALL/bin

set -x BUN_INSTALL $HOME/etc/bun
fish_add_path $BUN_INSTALL/bin
