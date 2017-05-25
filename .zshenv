autoload -U compinit; compinit
setopt auto_cd
setopt auto_pushd
setopt pushd_ignore_dups
setopt extended_glob
export EDITOR=vim
setopt auto_list
setopt auto_menu
setopt list_packed
setopt list_types
bindkey "^[[Z" reverse-menu-complete
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
unsetopt caseglob
setopt extended_history
#alias ruby='ruby -Ku'
autoload zmv
alias zmv='noglob zmv -W'
export PATH=$HOME/.local/bin:$PATH
