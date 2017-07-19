export PATH=$PATH:$HOME/bin
which /usr/libexec/java_home 1>/dev/null 2>&1 && export JAVA_HOME=$(/usr/libexec/java_home)
export PATH=$PATH:$HOME/Library/Activator/activator-dist-1.3.12/bin
export PYENV_ROOT=$HOME/.pyenv
export PATH=$PYENV_ROOT/bin:$PATH
which pyenv 1>/dev/null 2>&1 && eval "$(pyenv init -)"
export XDG_CONFIG_HOME=$HOME/.config
export PATH=$HOME/.rbenv/shims:$PATH
export PATH=$HOME/.rbenv/bin:$PATH
which rbenv 1>/dev/null 2>&1 && eval "$(rbenv init -)"
alias v="vim"
alias n="nvim"
alias alert="printf '\a'"
alias activo="cd $HOME/Activo/activo/documents; vagrant up; vagrant ssh"
export PATH=$HOME/Activo/activo/bin:$PATH
which hub 1>/dev/null 2>&1 && eval "$(hub alias -s)"
export XDG_CONFIG_HOME=$HOME/.config
alias ghc='stack ghc --'
alias ghci='stack ghci --'
alias runhaskell='stack runhaskell --'
alias py="python"
alias vimrc="$EDITOR ~/.vimrc"
alias zshrc="$EDITOR ~/.zshrc"
alias zshenv="$EDITOR ~/.zshenv"
alias zprofile="$EDITOR ~/.zprofile"
alias dein="$EDITOR ~/.dein.toml"
alias deinl="$EDITOR ~/.lazy_dain.toml"
alias pryrc="$EDITOR ~/.pryrc"
alias latexrc="$EDITOR ~/.latexmkrc"
alias col256='for c in {000..255}; do echo -n "\e[38;5;${c}m $c" ; [ $(($c%16)) -eq 15 ] && echo;done'
alias cons="rails c --sandbox 2>/dev/null"
if [ -d /var/www/html ] ; then
  alias home="/var/www/html"
  alias start="sudo redis-server /etc/redis.conf;sudo nginx -c /var/www/html/documents/nginx.conf;bundle exec unicorn_rails -c /var/www/html/config/unicorn.rb"
  alias refresh="sh /var/www/html/documents/dev/setup.sh"
  alias import_test_data="sh /var/www/html/documents/dev/import_test_data.sh"
  if [ ! "${IS_LOGIN_SHELL}" ] ; then
    pathmunge () {
      case ":${PATH}:" in
        *:"$1":*)
          ;;
        *)
          if [ "$2" = "after" ] ; then
            PATH=$PATH:$1
          else
            PATH=$1:$PATH
          fi
      esac
    }

    if [ $UID -gt 199 ] && [ "`id -gn`" = "`id -un`" ] ; then
      umask 002
    else
      umask 022
    fi

    for i in /etc/profile.d/*.sh; do
      if [ -r "$i" ] ; then
        if [ "$PS1" ] ; then
          . "$i"
        else
          . "$i" 1>/dev/null 2>&1
        fi
      fi
    done

    unset i; unset pathmunge
  fi
fi
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
cat $HOME/.ryuou
echo "\e[48;5;16m\e[38;5;7m+--------------------------------+\e[0m"
echo "\e[48;5;16m\e[38;5;7m| ようこそ 素晴らしき zshへ！  ▾ |\e[0m"
echo "\e[48;5;16m\e[38;5;7m+--------------------------------+\e[0m"
PROMPT="%(?!%F{119}!%F{123})%(?!${KAOMOJI_SUCCEED}!${KAOMOJI_FAIL})@%~::%f%F{69}%BInsert%b%f< "
SPROMPT="%F{212}%{$suggest%}${KAOMOJI_SUGGEST} < もしかして... %B%r%b %F{212}かな? [${EMOJI_YES}(y), ${EMOJI_NO}(n), ${EMOJI_ABORT}(a), ${EMOJI_EDIT}(e)]: %f"
