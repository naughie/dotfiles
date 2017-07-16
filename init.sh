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
rm -r ${NVIM_CONFIG}/ftdetect 2>/dev/null
rm -r ${NVIM_CONFIG}/colors 2>/dev/null
mkdir ${NVIM_CONFIG}/ftdetect 2>/dev/null
mkdir ${NVIM_CONFIG}/colors 2>/dev/null
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

echo '今回作成したシンボリックリンクはこちら！'
echo  "$HOME/.zshrc -> ${CONFIG_DIR}/.zshrc"
echo  "$HOME/.zshenv -> ${CONFIG_DIR}/.zshenv"
echo  "$HOME/.zprofile -> ${CONFIG_DIR}/.zprofile"
echo  "$HOME/.vimrc -> ${CONFIG_DIR}/.vimrc"
echo  "$HOME/.dein.toml -> ${CONFIG_DIR}/dein.toml"
echo  "$HOME/.lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo  "$HOME/.latexmkrc -> ${CONFIG_DIR}/.latexmkrc"
echo  "$HOME/.pryrc -> ${CONFIG_DIR}/.pryrc"
echo  "${NVIM_CONFIG}/init.vim -> ${CONFIG_DIR}/.vimrc"
echo  "${NVIM_CONFIG}/dein.toml -> ${CONFIG_DIR}/dein.toml"
echo  "${NVIM_CONFIG}/lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml"
echo  "${NVIM_CONFIG}/ftdetect -> ${CONFIG_DIR}/ftdetect"
echo  "${NVIM_CONFIG}/colors -> ${CONFIG_DIR}/colors"

if [ ${has_zsh} -eq 0 ] ; then
  if [ $SHELL != '/bin/zsh' ] ; then
    chsh -s /bin/zsh
  fi
  zsh
fi
