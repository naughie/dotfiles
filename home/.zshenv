export PATH=$HOME/.bin:$HOME/bin:/usr/local/sbt/bin:/usr/local/ssl/bin:$HOME/.local/bin:$HOME/.local/bin:/usr/local/bin:$PATH:/bin:/opt/X11/bin
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
# ENVs
source $HOME/.zshexp
# aliases
source $HOME/.zshalias
setopt no_global_rcs
