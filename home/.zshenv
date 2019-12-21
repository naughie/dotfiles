export PATH=$HOME/.bin:$HOME/bin::$HOME/.local/bin:/usr/local/bin:$PATH:/bin:/opt/X11/bin
case "$(uname -s)" in
  Linux*)  operating_system=Linux;;
  Darwin*) operating_system=Mac;;
  CYGWIN*) operating_system=Cygwin;;
  MINGW*)  operating_system=MinGW;;
  *)       operating_system="$(uname -s)";;
esac
# ENVs
source $HOME/.zshexp
# aliases
source $HOME/.zshalias
rundock ()
{
  docker-compose run --rm web bash -c "{ $* 3>&2 2>&1 1>&3 | grep --line-buffered -v '\\(/.*/.*.rb:.*:\\swarning:\\s\\|DEPRECATION\\sWARNING:\\s\\)'; exit \${PIPESTATUS[0]}; } 3>&2 2>&1 1>&3"
  command -v terminal-notifier >/dev/null 2>&1 && terminal-notifier -message 'Finish docker-compose run'
}
app_build()
{
  docker-compose build
  docker-compose run --rm web bundle exec rake db:create
  docker-compose run --rm web bundle exec rake db:migrate
}
dockb()
{
  rundock bundle exec $*
}
setopt no_global_rcs
