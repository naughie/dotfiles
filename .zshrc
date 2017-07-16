export PATH=$PATH:$HOME/bin
export TWITTER_CONSUMER_KEY=Q69JFxKponHTstUOKRJU93nJl
export TWITTER_CONSUMER_SECRET=IPlsxW2gJ8LpO7LytyEf1G3EedcQO3VfgnmCM3eRkNtk9lHhbN
export MAUTIC_PUBLIC_KEY='1_5lcng9h4j4840ck8gsowsgo0kcsc0cc4ckco4c4888cswsggkg'
export MAUTIC_SECRET_KEY='4wq5x3sbga2owco4sw0c0k4ss844cco4cskccw08kg4o4gow4k'
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
  if [ "${IS_LOGIN_SHELL}" ] ; then
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
