export PATH=$HOME/bin:/usr/local/sbt/bin:/usr/local/ssl/bin:/usr/local/zsh-5.8/bin:/usr/local/bin:$PATH:/bin:/opt/X11/bin
export MANPATH=/usr/local/zsh-5.8/share/man:$MANPATH
case "$(uname -s)" in
  Linux*)  operating_system=Linux;;
  Darwin*) operating_system=Mac;;
  CYGWIN*) operating_system=Cygwin;;
  MINGW*)  operating_system=MinGW;;
  *)       operating_system="$(uname -s)";;
esac
if [ "$operating_system" = "Mac" ]; then
  export PATH=/Library/TeX/texbin:$PATH
fi
ulimit -n 262144
# ENVs
source $HOME/.zshexp
# aliases
source $HOME/.zshalias
setopt no_global_rcs
