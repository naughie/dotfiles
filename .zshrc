[ -d "$HOME/.zplug" ] || curl -sL --proto-redir -all,https https://raw.githubusercontent.com/zplug/installer/master/installer.zsh | zsh
source $HOME/.zplug/init.zsh
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
setopt transient_rprompt
unsetopt caseglob
bindkey -v
autoload -U compinit; compinit
autoload -U zmv
alias zmv='noglob zmv -W'
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
function history-all { history -E 1 }

zplug "zsh-users/zsh-syntax-highlighting", defer:2
zplug "chrissicool/zsh-256color"
zplug "mollifier/cd-gitroot"
zplug "zsh-users/zsh-completions"
zplug "hchbaw/opp.zsh"

if ! zplug check --verbose; then
  printf "Install? [y/N]: "
  if read -q; then
      echo; zplug install
  fi
fi
zplug load --verbose

EMOJI_YES=$'\u2B55 '
EMOJI_NO=$'\u274C '
EMOJI_ABORT=$'\U1F44B '
EMOJI_EDIT=$'\u270F '
KAOMOJI_SUCCEED="(๑･𝗏･)و"$'\u2728 '
KAOMOJI_FAIL="(๑>﹏<%)"$'\U1F32A '
KAOMOJI_SUGGEST="(๑'~'%)"$'\u2753\uFE0F '
COLOR_SUCCESS='216'
COLOR_FAILURE='151'
COLOR_INSERTM='202'
COLOR_NORMALM='192'
COLOR_SUGGEST='212'
COLOR_RPROMPT='154'
function zle-line-init zle-keymap-select {
  PROMPT="%(?!%F{%{${COLOR_SUCCESS}}%}!%F{%{${COLOR_FAILURE}}%})%(?!${KAOMOJI_SUCCEED}!${KAOMOJI_FAIL})@%~::%f"
  SPROMPT="%F{%{${COLOR_SUGGEST}}%}%{$suggest%}${KAOMOJI_SUGGEST} < もしかして... %B%r%b %F{%{${COLOR_SUGGEST}}%}かな? [${EMOJI_YES}(y), ${EMOJI_NO}(n), ${EMOJI_ABORT}(a), ${EMOJI_EDIT}(e)]: %f"
  RPROMPT="%F{%{${COLOR_RPROMPT}}%}[%n 20%D %T]%f"
  case $KEYMAP in
    vicmd)
    PROMPT=$PROMPT"%F{%{${COLOR_NORMALM}}%}%BNormal%b%f< "
    ;;
    main|viins)
    PROMPT=$PROMPT"%F{%{${COLOR_INSERTM}}%}%BInsert%b%f< "
    ;;
  esac
  zle reset-prompt
}
zle -N zle-line-init
zle -N zle-keymap-select
