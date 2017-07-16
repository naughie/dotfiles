export LANG=ja_JP.UTF-8
setopt auto_cd
setopt auto_pushd
setopt pushd_ignore_dups
setopt extended_glob
which nvim 1>/dev/null 2>&1
if [ $? ] ; then
  export EDITOR=nvim
else
  export EDITOR=vim
fi
setopt auto_list
setopt auto_menu
setopt list_packed
setopt list_types
setopt prompt_subst
setopt correct
setopt nolistbeep
setopt extended_history
unsetopt caseglob
bindkey -v
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
autoload -U compinit; compinit
autoload zmv
alias zmv='noglob zmv -W'
export PATH=$HOME/.local/bin:$PATH
export KEYTIMEOUT=1
EMOJI_YES=$'\u2B55 '
EMOJI_NO=$'\u274C '
EMOJI_ABORT=$'\U1F44B '
EMOJI_EDIT=$'\u26D4 '
KAOMOJI_SUCCEED="(๑･𝗏･)و"$'\u2728 '
KAOMOJI_FAIL="(๑>﹏<%)"$'\U1F32A '
KAOMOJI_SUGGEST="(๑'~'%)"$'\u2753 '
function zle-line-init zle-keymap-select {
  PROMPT="%(?!%F{119}!%F{123})%(?!${KAOMOJI_SUCCEED}!${KAOMOJI_FAIL})@%~::%f"
  case $KEYMAP in
    vicmd)
    PROMPT=$PROMPT"%F{1}%BNormal%b%f< "
    ;;
    main|viins)
    PROMPT=$PROMPT"%F{69}%BInsert%b%f< "
    ;;
  esac
  zle reset-prompt
}
SPROMPT="%F{212}%{$suggest%}${KAOMOJI_SUGGEST} < もしかして... %B%r%b %F{212}かな? [${EMOJI_YES}(y), ${EMOJI_NO}(n), ${EMOJI_ABORT}(a), ${EMOJI_EDIT}(e)]: %f"
zle -N zle-line-init
zle -N zle-keymap-select
