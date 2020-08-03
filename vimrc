if &compatible
  set nocompatible
endif

augroup MyAutoCmd
  autocmd!
augroup END

let s:cache_home = empty($XDG_CACHE_HOME) ? expand('~/.cache') : $XDG_CACHE_HOME
let s:dein_dir = s:cache_home . '/dein'
let s:dein_repo_dir = s:dein_dir . '/repos/github.com/Shougo/dein.vim'

if !isdirectory(s:dein_repo_dir)
  call system('git clone https://github.com/Shougo/dein.vim ' . shellescape(s:dein_repo_dir))
endif
let &runtimepath = s:dein_repo_dir .",". &runtimepath

let s:toml_file = fnamemodify(expand('<sfile>'), ':h').'/dein.toml'
let s:lazy_toml_file = fnamemodify(expand('<sfile>'), ':h').'/lazy_dein.toml'
if dein#load_state(s:dein_dir)
  call dein#begin(s:dein_dir)
  call dein#load_toml(s:toml_file, {'lazy': 0})
  call dein#load_toml(s:lazy_toml_file, {'lazy': 1})
  call dein#end()
  call dein#save_state()
endif

if has('vim_starting') && dein#check_install()
  call dein#install()
endif

let g:python_host_prog  = $PYENV_ROOT . '/versions/2.7.18/bin/python'
let g:python3_host_prog = $PYENV_ROOT . '/versions/3.8.3/bin/python'

let s:config_home = empty($XDG_CONFIG_HOME) ? expand('~/.config') : $XDG_CONFIG_HOME
exec "source " . s:config_home . "/nvim/keymaps/main.vim"

set fencs=utf-8,euc-jp,default

set mouse-=a
set sh=zsh

set termguicolors

syntax on

filetype plugin indent on

set number
set ruler

set spelllang=en,cjk

set expandtab
set tabstop=4
set shiftwidth=2
set softtabstop=2
set autoindent
set smartindent

set pumheight=10

set showmatch
set matchtime=1

set nohlsearch

set ignorecase
set smartcase
set wildignorecase

set scrolloff=25

set display=lastline
set wrap

set conceallevel=0

set backspace=indent,eol,start

set laststatus=2
set statusline& statusline+=%#StatusLineFilename#%{'٩(๑´3｀)۶@'}%F
set statusline+=%m
set statusline+=%#warningmsg#
set statusline+=%=
set statusline+=%#StatusLineCursorPosition#(%l,%c)/(%L,%{strwidth(getline('.'))})
set statusline+=%{fugitive#statusline()}

set noswapfile
set backupskip=/tmp/*,/private/tmp/*

set nopaste

set vb t_vb=

set guicursor=a:hor5,a:blinkon500,a:blinkoff500,a:blinkwait0

set ambiwidth=double

set tags=tags,vendor.tags

set cursorline
hi clear CursorLine

augroup QfGroup
  au!
  au QuickFixCmdPost *grep* cwindow
augroup END

augroup TrailingSpace
  au!
  au VimEnter,WinEnter,ColorScheme * highlight link TrailingSpaces Error
  au VimEnter,WinEnter * match TrailingSpaces /\s\+$/
augroup END

augroup TwitVimSetting
  au!
  au FileType twitvim set wrap
augroup END

hi StatusLineFilename ctermfg=46 guifg=#00ff00
hi StatusLineCursorPosition ctermfg=184 guifg=#d7d700

" let g:tex_conceal = ''

let g:markdown_syntax_conceal = ''

let g:vim_markdown_folding_disabled=1

let g:unite_enable_start_insert=1

augroup ReadmeFiletype
  autocmd!
  autocmd BufRead,BufNewFile README.md set filetype=markdown.readme
augroup END
