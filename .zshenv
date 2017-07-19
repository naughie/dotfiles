export PATH=$PATH:/bin
export LANG=ja_JP.UTF-8
setopt auto_cd
setopt auto_pushd
setopt pushd_ignore_dups
setopt extended_glob
setopt auto_list
setopt auto_menu
setopt list_packed
setopt list_types
setopt prompt_subst
setopt correct
setopt nolistbeep
setopt extended_history
setopt hist_ignore_all_dups
setopt hist_save_no_dups
setopt no_beep
setopt no_hist_beep
setopt no_list_beep
setopt magic_equal_subst
unsetopt caseglob
bindkey -v
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
autoload -U compinit; compinit
autoload zmv
alias zmv='noglob zmv -W'
which nvim 1>/dev/null 2>&1
if [ $? -eq 0 ] ; then
  export EDITOR=nvim
else
  export EDITOR=vim
fi
export PATH=$HOME/.local/bin:$PATH
export KEYTIMEOUT=1
export CLICOLOR=true
export LSCOLORS='exfxcxdxbxGxDxabagacad'
export LS_COLORS='di=34:ln=35:so=32:pi=33:ex=31:bd=36;01:cd=33;01:su=31;40;07:sg=36;40;07:tw=32;40;07:ow=33;40;07:'
export HISTFILE=~/.zhistory
