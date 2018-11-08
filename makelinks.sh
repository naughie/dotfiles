#!/bin/bash

CONFIG_DIR=$PWD
NVIM_CONFIG=${XDG_CONFIG_HOME="$HOME/.config"}/nvim
mkdir -p ${NVIM_CONFIG} 2>/dev/null
mkdir -p ${XDG_CONFIG_HOME}/terminfo 2>/dev/null

#===== REMOVE OLD SYMLINKS =====
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
rm -r ${NVIM_CONFIG}/keymaps 2>/dev/null

#===== CREATE NEW SYMLINKS =====
#===== ZSH =====
ln -s ${CONFIG_DIR}/zshrc $HOME/.zshrc
ln -s ${CONFIG_DIR}/zshenv $HOME/.zshenv
ln -s ${CONFIG_DIR}/zprofile $HOME/.zprofile

#===== VIM =====
ln -s ${CONFIG_DIR}/vimrc $HOME/.vimrc
ln -s ${CONFIG_DIR}/dein.toml $HOME/.dein.toml
ln -s ${CONFIG_DIR}/lazy_dein.toml $HOME/.lazy_dein.toml
ln -s ${CONFIG_DIR}/vimrc ${NVIM_CONFIG}/init.vim
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
ln -s ${CONFIG_DIR}/keymaps ${NVIM_CONFIG}/keymaps

#===== TERMINFO =====
ln -s ${CONFIG_DIR}/xterm-256color-italic.terminfo ${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo
ln -s ${CONFIG_DIR}/screen-256color-italic.terminfo ${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo

#===== OTHERS =====
ln -s ${CONFIG_DIR}/latexmkrc $HOME/.latexmkrc
ln -s ${CONFIG_DIR}/pryrc $HOME/.pryrc
ln -s ${CONFIG_DIR}/tmux.conf $HOME/.tmux.conf

cat << EOF
今回作成したシンボリックリンクはこちら！

===== ZSH =====
$HOME/.zshrc -> ${CONFIG_DIR}/zshrc
$HOME/.zshenv -> ${CONFIG_DIR}/zshenv
$HOME/.zprofile -> ${CONFIG_DIR}/zprofile

===== VIM =====
$HOME/.vimrc -> ${CONFIG_DIR}/vimrc
$HOME/.dein.toml -> ${CONFIG_DIR}/dein.toml
$HOME/.lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml
${NVIM_CONFIG}/init.vim -> ${CONFIG_DIR}/vimrc
${NVIM_CONFIG}/dein.toml -> ${CONFIG_DIR}/dein.toml
${NVIM_CONFIG}/lazy_dein.toml -> ${CONFIG_DIR}/lazy_dein.toml
${NVIM_CONFIG}/ftdetect -> ${CONFIG_DIR}/ftdetect
${NVIM_CONFIG}/colors -> ${CONFIG_DIR}/colors
${NVIM_CONFIG}/syntax -> ${CONFIG_DIR}/syntax
${NVIM_CONFIG}/indent -> ${CONFIG_DIR}/indent
${NVIM_CONFIG}/ftplugin -> ${CONFIG_DIR}/ftplugin
${NVIM_CONFIG}/snippets -> ${CONFIG_DIR}/snippets
${NVIM_CONFIG}/autoload -> ${CONFIG_DIR}/autoload
${NVIM_CONFIG}/template -> ${CONFIG_DIR}/template
${NVIM_CONFIG}/keymaps -> ${CONFIG_DIR}/keymaps

===== TERMINFO =====
${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo ${CONFIG_DIR}/xterm-256color-italic.terminfo
${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo ${CONFIG_DIR}/screen-256color-italic.terminfo

===== OTHERS =====
$HOME/.latexmkrc -> ${CONFIG_DIR}/latexmkrc
$HOME/.pryrc -> ${CONFIG_DIR}/pryrc
$HOME/.tmux.conf -> ${CONFIG_DIR}/tmux.conf

===== END =====
EOF

tic ${XDG_CONFIG_HOME}/terminfo/xterm-256color-italic.terminfo
tic ${XDG_CONFIG_HOME}/terminfo/screen-256color-italic.terminfo

