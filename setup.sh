#/bin/sh

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

CONFIG_DIR=$PWD
NVIM_CONFIG=${XDG_CONFIG_HOME="$HOME/.config"}/nvim
mkdir -p ${NVIM_CONFIG} 2>/dev/null
mkdir -p ${XDG_CONFIG_HOME}/terminfo 2>/dev/null
rm $HOME/.zshrc 2>/dev/null
rm $HOME/.zshenv 2>/dev/null
rm $HOME/.zprofile 2>/dev/null
rm $HOME/.vimrc 2>/dev/null
rm $HOME/.dein.toml 2>/dev/null
rm $HOME/.lazy_dein.toml 2>/dev/null
rm $HOME/.latexmkrc 2>/dev/null
rm $HOME/.pryrc 2>/dev/null
rm $HOME/.tmux.conf 2>/dev/null
rm ${NVIM_CONFIG}/terminfo/xterm-256color-italic.terminfo 2>/dev/null
rm ${NVIM_CONFIG}/terminfo/screen-256color-italic.terminfo 2>/dev/null
rm ${NVIM_CONFIG}/init.vim 2>/dev/null
rm ${NVIM_CONFIG}/dein.toml 2>/dev/null
rm ${NVIM_CONFIG}/lazy_dein.toml 2>/dev/null
rm -r ${NVIM_CONFIG}/ftdetect 2>/dev/null
rm -r ${NVIM_CONFIG}/colors 2>/dev/null
rm -r ${NVIM_CONFIG}/syntax 2>/dev/null
rm -r ${NVIM_CONFIG}/indent 2>/dev/null
rm -r ${NVIM_CONFIG}/ftplugin 2>/dev/null
rm -r ${NVIM_CONFIG}/snippets 2>/dev/null
rm -r ${NVIM_CONFIG}/autoload 2>/dev/null
rm -r ${NVIM_CONFIG}/template 2>/dev/null
ln -s ${CONFIG_DIR}/.zshrc $HOME/.zshrc
ln -s ${CONFIG_DIR}/.zshenv $HOME/.zshenv
ln -s ${CONFIG_DIR}/.zprofile $HOME/.zprofile
ln -s ${CONFIG_DIR}/.vimrc $HOME/.vimrc
ln -s ${CONFIG_DIR}/dein.toml $HOME/.dein.toml
ln -s ${CONFIG_DIR}/lazy_dein.toml $HOME/.lazy_dein.toml
ln -s ${CONFIG_DIR}/.vimrc ${NVIM_CONFIG}/init.vim
ln -s ${CONFIG_DIR}/dein.toml ${NVIM_CONFIG}/dein.toml
ln -s ${CONFIG_DIR}/lazy_dein.toml ${NVIM_CONFIG}/lazy_dein.toml
ln -s ${CONFIG_DIR}/ftdetect ${NVIM_CONFIG}/ftdetect
ln -s ${CONFIG_DIR}/colors ${NVIM_CONFIG}/colors
ln -s ${CONFIG_DIR}/syntax ${NVIM_CONFIG}/syntax
ln -s ${CONFIG_DIR}/indent ${NVIM_CONFIG}/indent
ln -s ${CONFIG_DIR}/ftplugin ${NVIM_CONFIG}/ftplugin
ln -s ${CONFIG_DIR}/snippets ${NVIM_CONFIG}/snippets
ln -s ${CONFIG_DIR}/autoload ${NVIM_CONFIG}/autoload
ln -s ${CONFIG_DIR}/template ${NVIM_CONFIG}/template
ln -s ${CONFIG_DIR}/.latexmkrc $HOME/.latexmkrc
ln -s ${CONFIG_DIR}/.pryrc $HOME/.pryrc
ln -s ${CONFIG_DIR}/.tmux.conf $HOME/.tmux.conf
ln -s ${CONFIG_DIR}/xterm-256color-italic.terminfo ${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo
ln -s ${CONFIG_DIR}/screen-256color-italic.terminfo ${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo

echo '今回作成したシンボリックリンクはこちら！'
echo "$HOME/.zshrc -> ${CONFIG_DIR}/.zshrc"
echo "$HOME/.zshenv -> ${CONFIG_DIR}/.zshenv"
echo "$HOME/.zprofile -> ${CONFIG_DIR}/.zprofile"
echo "$HOME/.vimrc -> ${CONFIG_DIR}/.vimrc"
echo "$HOME/.dein.toml -> ${CONFIG_DIR}/dein.toml"
echo "$HOME/.lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo "$HOME/.latexmkrc -> ${CONFIG_DIR}/.latexmkrc"
echo "$HOME/.pryrc -> ${CONFIG_DIR}/.pryrc"
echo "$HOME/.tmux.conf -> ${CONFIG_DIR}/.tmux.conf"
echo "${NVIM_CONFIG}/init.vim -> ${CONFIG_DIR}/.vimrc"
echo "${NVIM_CONFIG}/dein.toml -> ${CONFIG_DIR}/dein.toml"
echo "${NVIM_CONFIG}/lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo "${NVIM_CONFIG}/ftdetect -> ${CONFIG_DIR}/ftdetect"
echo "${NVIM_CONFIG}/colors -> ${CONFIG_DIR}/colors"
echo "${NVIM_CONFIG}/syntax -> ${CONFIG_DIR}/syntax"
echo "${NVIM_CONFIG}/indent -> ${CONFIG_DIR}/indent"
echo "${NVIM_CONFIG}/ftplugin -> ${CONFIG_DIR}/ftplugin"
echo "${NVIM_CONFIG}/snippets -> ${CONFIG_DIR}/snippets"
echo "${NVIM_CONFIG}/autoload -> ${CONFIG_DIR}/autoload"
echo "${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo ${CONFIG_DIR}/xterm-256color-italic.terminfo"
echo "${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo ${CONFIG_DIR}/screen-256color-italic.terminfo"

tic ${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo
tic ${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo

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
