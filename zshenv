export PATH=$HOME/.bin:$HOME/bin:/Library/TeX/texbin:/Applications/Wireshark.app/Contents/MacOS:/usr/local/twitter/bin:/usr/local/sbt/bin:/usr/local/ssl/bin:$HOME/.local/bin:$HOME/.local/bin:/usr/local/bin:$PATH:/bin:/opt/X11/bin
export PATH="/usr/local/opt/openssl/bin:$PATH"
case "$(uname -s)" in
  Linux*)  operating_system=Linux;;
  Darwin*) operating_system=Mac;;
  CYGWIN*) operating_system=Cygwin;;
  MINGW*)  operating_system=MinGW;;
  *)       operating_system="$(uname -s)";;
esac
export LANG=ja_JP.UTF-8
export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
export CLICOLOR=true
export KEYTIMEOUT=1
export DROPBOX=/Volumes/Samsung/Dropbox
# zplug env
export ZPLUG_HOME=$HOME/.zplug
export ZPLUG_ROOT=$HOME/.zplug
# zsh history
export HISTFILE=~/.zhistory
export SAVEHIST=200
# set EDITOR to either nvim or vim
command -v nvim >/dev/null &&
  export EDITOR=nvim ||
  export EDITOR=vim
# set terminal color
export TERM=xterm-256color-italic
# set config env
export XDG_CONFIG_HOME=$HOME/.config
export AWS_CONFIG_FILE=~/.aws
export TEXMFLOCAL=/usr/local/texlive/texmf-local
# init java
which /usr/libexec/java_home 1>/dev/null 2>&1 && export JAVA_HOME=$(/usr/libexec/java_home)
# init pyenv
export PYENV_ROOT=$HOME/.pyenv
export PATH=$PYENV_ROOT/bin:$PATH
command -v pyenv >/dev/null && eval "$(pyenv init -)"
# command -v pyenv >/dev/null && eval "$(pyenv virtualenv-init -)"
test -r $PYENV_ROOT/versions/neovim2/bin/activate && source $PYENV_ROOT/versions/neovim2/bin/activate
test -r $PYENV_ROOT/versions/neovim3/bin/activate && source $PYENV_ROOT/versions/neovim3/bin/activate
# init rbenv
export RBENV_ROOT=$HOME/.rbenv
export PATH=$RBENV_ROOT/bin:$PATH
command -v rbenv >/dev/null && eval "$(rbenv init -)"
# enable nodebrew
export PATH=$HOME/.nodebrew/current/bin:$PATH
# enable rust
export PATH=$HOME/.cargo/bin:$PATH
# enable ocaml/opam
test -r $HOME/.opam/opam-init/init.zsh && . $HOME/.opam/opam-init/init.zsh > /dev/null 2> /dev/null || true
# aliases
alias v="vim -u $HOME/.vimrc"
alias n="nvim -u $XDG_CONFIG_HOME/nvim/init.vim"
alias vs="sudo vim -u $HOME/.vimrc"
alias ns="sudo nvim -u $XDG_CONFIG_HOME/nvim/init.vim"
alias alert="printf '\a'"
alias ghc='stack ghc'
alias ghci='stack ghci'
alias py="python"
alias vimrc="$EDITOR ~/.vimrc"
alias zshrc="$EDITOR ~/.zshrc"
alias zshenv="$EDITOR ~/.zshenv"
alias zprofile="$EDITOR ~/.zprofile"
alias dein="$EDITOR ~/.dein.toml"
alias deinl="$EDITOR ~/.lazy_dein.toml"
alias pryrc="$EDITOR ~/.pryrc"
alias latexrc="$EDITOR ~/.latexmkrc"
alias netrc="$EDITOR ~/.netrc"
alias tconf="$EDITOR ~/.tmux.conf"
alias curld="curl --trace-ascii /dev/stderr "
alias reboot="sudo shutdown -r now"
alias gpp='g++ -std=c++11'
alias gpps='g++ -std=c++11 -lssl -lcrypto -lcurl'
alias eject='diskutil eject'
alias gitcd='cd-gitroot'
alias grep='grep -i'
alias ocaml='rlwrap ocaml'
alias less='less -r'
if [ "${operating_system}" = "Mac" ]; then
  alias off="osascript -e 'tell app \"Finder\" to sleep'"
  alias chrome='open -a "Google Chrome"'
  alias finder='open -a Finder'
  alias saf='open -a Safari'
  alias notify='terminal-notifier -message'
fi
function nt () {
  fileName=${1%\.tex}
  fileName=${fileName%\.}
  nvim ${fileName}.tex
}
alias col256='for c in {000..255}; do echo -n "\e[38;5;${c}m $c" ; [ $(($c%16)) -eq 15 ] && echo;done'
alias cons="rails c --sandbox 2>/dev/null"
alias cond='rundock bundle exec rails console --sandbox'
alias migr='rundock bundle exec rails db:migrate'
alias bund='rundock bundle install'
alias tailog="grep --line-buffered -v '\(\(Article\|Picture\|Area\|ActivityDate\) Load (\d*\.\d*ms)\|(\d*\.\d*ms).*SELECT.*\`article.*\`\)'"
rundock ()
{
  docker-compose run --rm web bash -c "{ $* 3>&2 2>&1 1>&3 | grep --line-buffered -v '\\(/.*/.*.rb:.*:\\swarning:\\s\\|DEPRECATION\\sWARNING:\\s\\)'; exit \${PIPESTATUS[0]}; } 3>&2 2>&1 1>&3"
  command -v terminal-notifier >/dev/null 2>&1 && terminal-notifier -message 'Finish docker-compose run'
}
app_build()
{
  docker-compose build
  docker-compose run --rm web bundle exec rake db:create
  docker-compose run --rm web bundle exec rake db:migrate
}
dockb()
{
  rundock bundle exec $*
}
setopt no_global_rcs
