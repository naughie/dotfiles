export ICLOUD="/Users/nakatam/Library/Mobile\ Documents/com~apple~CloudDocs/"
export TWITTER_CONSUMER_KEY=Q69JFxKponHTstUOKRJU93nJl
export TWITTER_CONSUMER_SECRET=IPlsxW2gJ8LpO7LytyEf1G3EedcQO3VfgnmCM3eRkNtk9lHhbN
export JAVA_HOME=$(/usr/libexec/java_home)
export PATH=$PATH:$HOME/Library/Activator/activator-dist-1.3.12/bin
export PYENV_ROOT=$HOME/.pyenv
export PATH=$PYENV_ROOT/bin:$PATH
eval "$(pyenv init -)"
export XDG_CONFIG_HOME=$HOME/.config
export PATH=$HOME/.rbenv/shims:$PATH
#export PATH=/Users/nakatam/.rbenv/versions/2.3.1/bin:$PATH
alias v="vim"
alias n="nvim"
alias alert="printf '\a'"
alias activo="cd $HOME/Activo/activo/documents; vagrant up; vagrant ssh"
export PATH=$HOME/Activo/activo/bin:$PATH
eval "$(hub alias -s)"
alias drop="cd Dropbox"
export XDG_CONFIG_HOME=$HOME/.config
alias atail="tail -f ~/Activo/activo/log/development.log | grep --line-buffered -v 'Article Exists'"
alias ghc='stack ghc --'
alias ghci='stack ghci --'
alias runhaskell='stack runhaskell --'
#alias ruby='bundle exec ruby'
#alias rails='bundle exec rails'
#alias rake='bundle exec rake'
