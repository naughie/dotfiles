#/bin/sh

which /bin/zsh 1>/dev/null 2>&1
has_zsh=$?
which nvim 1>/dev/null 2>&1
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
rm $HOME/.zshrc 2>/dev/null
rm $HOME/.zshenv 2>/dev/null
rm $HOME/.zprofile 2>/dev/null
rm $HOME/.vimrc 2>/dev/null
rm $HOME/.dein.toml 2>/dev/null
rm $HOME/.lazy_dein.toml 2>/dev/null
rm $HOME/.latexmkrc 2>/dev/null
rm $HOME/.pryrc 2>/dev/null
rm ${NVIM_CONFIG}/init.vim 2>/dev/null
rm ${NVIM_CONFIG}/dein.toml 2>/dev/null
rm ${NVIM_CONFIG}/lazy_dein.toml 2>/dev/null
rm $HOME/.ryujinou 2>/dev/null
rm -r ${NVIM_CONFIG}/ftdetect 2>/dev/null
rm -r ${NVIM_CONFIG}/colors 2>/dev/null
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
ln -s ${CONFIG_DIR}/.latexmkrc $HOME/.latexmkrc
ln -s ${CONFIG_DIR}/.pryrc $HOME/.pryrc
ln -s ${CONFIG_DIR}/images/ryujinou $HOME/.ryujinou

echo '今回作成したシンボリックリンクはこちら！'
echo "$HOME/.zshrc -> ${CONFIG_DIR}/.zshrc"
echo "$HOME/.zshenv -> ${CONFIG_DIR}/.zshenv"
echo "$HOME/.zprofile -> ${CONFIG_DIR}/.zprofile"
echo "$HOME/.vimrc -> ${CONFIG_DIR}/.vimrc"
echo "$HOME/.dein.toml -> ${CONFIG_DIR}/dein.toml"
echo "$HOME/.lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo "$HOME/.latexmkrc -> ${CONFIG_DIR}/.latexmkrc"
echo "$HOME/.pryrc -> ${CONFIG_DIR}/.pryrc"
echo "$HOME/.ryujinou -> ${CONFIG_DIR}/images/ryujinou"
echo "${NVIM_CONFIG}/init.vim -> ${CONFIG_DIR}/.vimrc"
echo "${NVIM_CONFIG}/dein.toml -> ${CONFIG_DIR}/dein.toml"
echo "${NVIM_CONFIG}/lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo "${NVIM_CONFIG}/ftdetect -> ${CONFIG_DIR}/ftdetect"
echo "${NVIM_CONFIG}/colors -> ${CONFIG_DIR}/colors"

which imgcat 1>/dev/null 2>&1
if [ $? -ne 0 ] ; then
  curl 'https://raw.githubusercontent.com/gnachman/iTerm2/master/tests/imgcat' -o imgcat
  chmod +x imgcat
  mv imgcat /usr/local/bin
  echo 'imgcatをdownloadしたよ！'
fi

if [ -f '/etc/centos-release' ] ; then
  if [ ${has_zsh} -ne 0 ] ; then
    sudo yum -y install zsh && echo 'zshをinstallしたよ！'
  fi
  if [ $SHELL != '/bin/zsh' ] ; then
    chsh -s /bin/zsh
  fi
  zsh --login
  which pyenv 1>/dev/null 2>&1
  has_pyenv=$?
  if [ ${has_pyenv} -ne 0 ] ; then
    sudo yum install gcc zlib-devel bzip2 bzip2-devel readline readline-devel sqlite sqlite-devel openssl openssl-devel git
    git clone https://github.com/yyuu/pyenv.git ~/.pyenv && 'pyenvが使えるようになったよ！'
    source $HOME/.zshrc
    pyenv install 3.6.0
    pyenv global 3.6.0 'pythonを3.6.0に設定したよ！'
  fi
  if [ ${has_nvim} -ne 0 ] ; then
    sudo yum -y install libtool autoconf automake cmake gcc gcc-c++ make pkgconfig unzip
    yum -y install epel-release
    curl -o /etc/yum.repos.d/dperson-neovim-epel-7.repo https://copr.fedorainfracloud.org/coprs/dperson/neovim/repo/epel-7/dperson-neovim-epel-7.repo
    yum -y install neovim && 'neovimをinstallしたよ！'
    export EDITOR=nvim
    echo '初期設定をするため，neovimを起動するよ！'
  fi
fi
