export PATH=$HOME/.bin:$HOME/bin:/Library/TeX/texbin:/Applications/Wireshark.app/Contents/MacOS:/usr/local/twitter/bin:/usr/local/sbt/bin:/usr/local/ssl/bin:$HOME/.local/bin:$HOME/.local/bin:/usr/local/bin:$PATH:/bin:/opt/X11/bin
export LANG=ja_JP.UTF-8
export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
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
command -v pyenv >/dev/null && eval "$(pyenv virtualenv-init -)"
# init rbenv
export RBENV_ROOT=$HOME/.rbenv
export PATH=$RBENV_ROOT/bin:$PATH
command -v rbenv >/dev/null && eval "$(rbenv init -)"
# enable nodebrew
export PATH=$HOME/.nodebrew/current/bin:$PATH
# enable rust
export PATH=$HOME/.cargo/bin:$PATH
# enable ocaml/opam
. ~/.opam/opam-init/init.zsh > /dev/null 2> /dev/null || true
command -v opam >/dev/null && eval "$(opam config env)"
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
alias off="osascript -e 'tell app \"Finder\" to sleep'"
alias gitcd='cd-gitroot'
alias grep='grep -i'
alias ocaml='rlwrap ocaml'
function nt () {
  fileName=${1%\.tex}
  fileName=${fileName%\.}
  nvim ${fileName}.tex
}
alias col256='for c in {000..255}; do echo -n "\e[38;5;${c}m $c" ; [ $(($c%16)) -eq 15 ] && echo;done'
alias cons="rails c --sandbox 2>/dev/null"
alias cond='rundock rails console --sandbox'
alias migr='rundock rails db:migrate'
alias bund='rundock bundle install --without production'
alias tailog="grep --line-buffered -v '\(\(Article\|Picture\|Area\|ActivityDate\) Load (\d*\.\d*ms)\|(\d*\.\d*ms).*SELECT.*\`article.*\`\)'"
rundock ()
{
  docker-compose run web bash -c "{ $* 3>&2 2>&1 1>&3 | grep --line-buffered -v '/.*/.*.rb:.*:\\swarning:\\s'; exit \${PIPESTATUS[0]}; } 3>&2 2>&1 1>&3"
}
setopt no_global_rcs
#green="\e[48;5;64m"
#gray="\e[48;5;247m"
#blue="\e[48;5;20m"
#orange="\e[48;5;172m"
#lblue="\e[48;5;38m"
#black="\e[48;5;16m"
#purple="\e[48;5;91m"
#brown="\e[48;5;130m"
#white="\e[48;5;7m"
#echo "${green}        ${gray}  ${green}              ${gray}  ${green}        ${gray}  \e[0m"
#echo "${green}      ${blue}    ${green}  ${orange}            ${green}  ${blue}    ${green}      \e[0m"
#echo "${green}      ${blue}          ${orange}    ${blue}          ${green}      \e[0m"
#echo "${gray}  ${green}    ${orange}  ${blue}                    ${orange}  ${green}      \e[0m"
#echo "${green}    ${orange}    ${blue}    ${lblue}  ${blue}        ${lblue}  ${blue}    ${brown}      ${green}  \e[0m"
#echo "${green}    ${orange}    ${white}  ${blue}  ${lblue}    ${blue}    ${lblue}    ${blue}  ${brown}    ${white}  ${brown}  ${green}  \e[0m"
#echo "${green}    ${orange}  ${white}    ${blue}  ${lblue}  ${black}  ${lblue}    ${black}  ${lblue}  ${blue}  ${white}  ${brown}      ${green}  \e[0m"
#echo "${green}      ${purple}  ${white}    ${lblue}            ${white}  ${brown}        ${green}  \e[0m"
#echo "${green}    ${purple}      ${white}  ${lblue}  ${black}        ${lblue}  ${white}  ${purple}    ${brown}  ${green}  ${gray}  \e[0m"
#echo "${green}    ${purple}  ${lblue}  ${purple}  ${white}    ${lblue}        ${white}    ${purple}  ${black}  ${brown}    ${green}  \e[0m"
#echo "${green}    ${purple}  ${lblue}      ${purple}  ${white}  ${purple}    ${white}  ${purple}    ${black}  ${purple}  ${brown}    ${green}  \e[0m"
#echo "${gray}  ${green}  ${lblue}      ${black}  ${purple}    ${white}    ${purple}    ${black}  ${purple}    ${brown}  ${lblue}  ${green}  \e[0m"
#echo "${green}  ${purple}    ${lblue}    ${black}  ${orange}  ${purple}  ${white}    ${purple}  ${orange}  ${purple}    ${lblue}      ${green}  \e[0m"
#echo "${green}  ${purple}  ${lblue}    ${black}  ${purple}    ${orange}        ${black}  ${purple}    ${black}  ${lblue}    ${green}  \e[0m"
#echo "${green}  ${purple}    ${lblue}  ${purple}  ${black}  ${purple}        ${black}  ${purple}    ${black}  ${purple}  ${brown}  ${purple}  ${green}  \e[0m"
#echo "${green}    ${black}  ${purple}      ${black}  ${purple}          ${black}  ${purple}    ${brown}  ${green}    \e[0m"
#echo "${green}  ${purple}        ${black}  ${purple}          ${black}  ${purple}      ${brown}  ${purple}  ${gray}  \e[0m"
#echo "${green}        ${gray}  ${green}              ${gray}  ${green}          \e[0m"
#cat $HOME/.ryujinou
