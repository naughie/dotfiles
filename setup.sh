#!/bin/bash

command -v zsh 1>/dev/null 2>&1
has_zsh=$?
command -v nvim 1>/dev/null 2>&1
has_nvim=$?
if [ ${has_zsh} -ne 0 ] ; then
  echo 'WARNING: zshがinstallされてないよ！'
fi
if [ ${has_nvim} -ne 0 ] ; then
  echo 'WARNING: neovimがinstallされてないよ！'
fi


command -v imgcat 1>/dev/null 2>&1 || {
  curl 'https://raw.githubusercontent.com/gnachman/iTerm2/master/tests/imgcat' -o imgcat
  chmod +x imgcat
  sudo mv imgcat /usr/local/bin
  echo 'imgcatをdownloadしたよ！'
}

if [ -f '/etc/centos-release' ] ; then
  echo '必要なpackagesをinstallするよ！'
  sudo yum -y install libtool autoconf automake cmake gcc gcc-c++ make pkgconfig unzip bzip2 bzip2-devel openssl openssl-devel readline readline-devel epel-release
fi
if [ -f '/etc/lsb-release' ] ; then
  echo '必要なpackagesをinstallするよ！'
  sudo apt-get -y install git gcc make openssl libssl-dev libbz2-dev libreadline-dev libsqlite3-dev software-properties-common python-software-properties
fi

if [ ${has_zsh} -ne 0 ] ; then
  if [ -f '/etc/centos-release' ] ; then
    sudo yum -y install zsh && echo 'zshをinstallしたよ！'
  fi
  if [ -f '/etc/lsb-release' ] ; then
    sudo apt-get install zsh && echo 'zshをinstallしたよ！'
  fi
  chsh -s $(which zsh)
  zsh --login
fi

command -v pyenv >/dev/null || {
  git clone https://github.com/pyenv/pyenv.git $HOME/.pyenv
  source $HOME/.zshrc
  pyenv install 2.7.14
  pyenv install 3.6.0 && pyenv global 3.6.0 && echo 'python 3.6.0をinstallしたよ！'
  git clone https://github.com/pyenv/pyenv-virtualenv.git $PYENV_ROOT/plugins/pyenv-virtualenv
  source $HOME/.zshrc
  pyenv virtualenv 2.7.14 neovim2
  pyenv virtualenv 3.6.0 neovim3
  pyenv activate neovim2
  pip install neovim
  pyenv activate neovim3
  pip install neovim
}

command -v rbenv >/dev/null || {
  git clone https://github.com/rbenv/rbenv.git $HOME/.rbenv
  mkdir -p $RBENV_ROOT/plugins
  git clone https://github.com/rbenv/ruby-build.git $RBENV_ROOT/plugins/ruby-build
  source $HOME/.zshrc
  rbenv install 2.5.0 && rbenv global 2.5.0 && echo 'ruby 2.5.0をinstallしたよ！'
  gem install neovim
}

if [ ${has_nvim} -ne 0 ] ; then
  if [ -f '/etc/centos-release' ] ; then
    curl -o /etc/yum.repos.d/dperson-neovim-epel-7.repo https://copr.fedorainfracloud.org/coprs/dperson/neovim/repo/epel-7/dperson-neovim-epel-7.repo
    yum -y install neovim && echo 'neovimをinstallしたよ！'
  fi
  if [ -f '/etc/lsb-release' ] ; then
    sudo add-apt-repository ppa:neovim-ppa/stable
    sudo apt-get update
    sudo apt-get install neovim && echo 'neovimをinstallしたよ！'
  fi
  source $HOME/.zshrc
  echo '初期設定をするため，neovimを起動するよ！'
  nvim
fi
