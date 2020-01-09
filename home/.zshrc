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
zplug "mollifier/cd-gitroot"
alias gitcd='cd-gitroot'
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
COLOR_COMMAND='251'
EMOJI_SUCCEED=$'\U1F60A'
EMOJI_FAILURE=$'\U1F62D'
function zle-line-init zle-keymap-select {
  printf "\a"
  # PROMPT=$'%(?!\U1F60A!\U1F62D)%(?!%F{%{216}%}!%F{%{151}%}) Last status code is %?!%f    %F{%{6}%}You are now "%n" and @ "%~"    %f'
  PROMPT=$'%(?!\U1F60A!\U1F62D)%(?!%F{%{216}%}!%F{%{151}%}) Last status code is %?!%f    %F{%{6}%}You are now \\\`%n\' and @ \\\`%~\'    %f'
  SPROMPT=$'%F{%{212}%}%{$suggest%}\U1F914 You may intend to ... %B%r%b [y(es), n(o), a(bort), e(dit)]: %f'
  RPROMPT=$'%F{%{154}%}[%D{%m/%d/%Y} %0(D.\U2603.%1(D.\U2603.%2(D.\U1F338.%3(D.\U1F338.%4(D.\U1F338.%5(D.\U2614.%6(D.\U1F33B.%7(D.\U1F33B.%8(D.\U1F341.%9(D.\U1F341.%10(D.\U1F341.\U2603))))))))))) %T]%f'
  case $KEYMAP in
    vicmd)
    PROMPT=$PROMPT$'%F{%{192}%}%BNormal%b%f'
    ;;
    main|viins)
    PROMPT=$PROMPT$'%F{%{202}%}%BInsert%b%f'
    ;;
    *)
    PROMPT=$PROMPT$'%F{%{192}%}%BNormal%b%f'
    ;;
  esac
  PROMPT=$PROMPT$'\n  %F{%{251}%}%BCommand? \U300B%b%f '
  zle reset-prompt
}
zle -N zle-line-init
zle -N zle-keymap-select
test -f $HOME/.zlogin && source $HOME/.zlogin
