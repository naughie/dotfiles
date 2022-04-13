let g:python_host_prog  = $HOME . '/etc/nvim-python/nvim-python2/bin/python2'
let g:python3_host_prog  = $HOME . '/etc/nvim-python/nvim-python3/bin/python3'

if has('nvim')

    "for dein cache
    let s:dein_dir = $HOME . '/.local/share/dein'
    let s:dein_repo_dir = s:dein_dir . '/repos/github.com/Shougo/dein.vim'

    " locate dein.toml/lazy_dein.toml
    let s:nvim_conf_home = $XDG_CONFIG_HOME . '/nvim'
    let s:toml_file = s:nvim_conf_home . '/dein.toml'
    let s:lazy_toml_file = s:nvim_conf_home . '/lazy_dein.toml'

    if !isdirectory(s:dein_repo_dir)
      call system('git clone https://github.com/Shougo/dein.vim ' . shellescape(s:dein_repo_dir))
    endif
    let &runtimepath = s:dein_repo_dir .",". &runtimepath

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

endif

syntax on

filetype plugin indent on

set fencs=utf-8,euc-jp,default

set mouse-=a

set termguicolors

set number
set ruler

set hidden

set spelllang=en_us,cjk

set expandtab
set tabstop=4
" sw is the same as `tabstop`
set shiftwidth=0
" sts is the same as `shiftwidth`
"set softtabstop=4
set autoindent
set smartindent

set pumheight=10

set showmatch
set matchtime=1

set nohlsearch

set ignorecase
set smartcase
set fileignorecase

set scrolloff=25

set display=lastline,uhex
set wrap

set conceallevel=0

set backspace=indent,eol,start

set laststatus=2
set statusline& statusline+=%#StatusLineFilename#%{'ヾ(๑╹◡╹)ﾉ\"@@@'}%f
set statusline+=%m
set statusline+=%#warningmsg#
set statusline+=%=
set statusline+=%#StatusLineCursorPosition#(%l,%c)/(%L,%{strwidth(getline('.'))})
"set statusline+=%{fugitive#statusline()}
hi StatusLineFilename ctermfg=46 guifg=#00ff00
hi StatusLineCursorPosition ctermfg=184 guifg=#d7d700


set noswapfile
set backupskip=/tmp/*,/private/tmp/*
set nobackup
set nowritebackup

set visualbell

set updatetime=300

set shortmess+=c

set guicursor=a:hor5,a:blinkon500,a:blinkoff500,a:blinkwait0

"set ambiwidth=double

set tags=./tags

set signcolumn=yes

set cursorline
hi clear CursorLine

nnoremap <silent> <BS> <C-w>h
nnoremap <silent> <C-h> <C-w>h
nnoremap <silent> <C-l> <C-w>l
nnoremap <silent> <C-k> <C-w>k
nnoremap <silent> <C-j> <C-w>j

nnoremap <silent> j gj
nnoremap <silent> k gk
nnoremap <silent> gj j
nnoremap <silent> gk k
vnoremap <silent> j gj
vnoremap <silent> k gk
vnoremap <silent> gj j
vnoremap <silent> gk k

nnoremap <silent> <Space>h ^
nnoremap <silent> <Space>l $
vnoremap <silent> <Space>h ^
vnoremap <silent> <Space>l $

nnoremap <Space>w :w<CR>
nnoremap <Space>q :q<CR>

vnoremap <silent> <Space>y "+y
nnoremap <silent> <Space>v <C-v>

nnoremap <silent> Y y$

nnoremap <C-p> "zdd<Up>"zP
nnoremap <C-n> "zdd"zp
vnoremap <C-p> "zx<Up>"zP`[V`]
vnoremap <C-n> "zx"zp`[V`]

nnoremap <silent> <Space><Space> "zyiw:let @/ = '\<' . @z . '\>'<CR>:set hlsearch<CR>
nnoremap <silent> <Space>n :noh<CR>

nnoremap <silent> n nzz
nnoremap <silent> N Nzz
nnoremap <silent> * *zz
nnoremap <silent> # #zz
nnoremap <silent> g* g*zz
nnoremap <silent> g# g#zz

cnoremap <expr> / getcmdtype() == '/' ? '\/' : '/'
cnoremap <expr> ? getcmdtype() == '?' ? '\?' : '?'

nnoremap <silent> <Space>j <C-]>

command TODO vim TODO **

"augroup TrailingSpace
"  au!
"  au VimEnter * hi link TrailingSpaces Error
"  au WinEnter,BufWinEnter,BufEnter * hi clear TrailingSpaces
"  "au VimEnter,BufWinEnter * call HighlightTrailingSpaces()
"augroup END
"
"function! HighlightTrailingSpaces() abort
"    if &filetype ==# 'defx'
"        highlight clear TrailingSpaces
"    else
"        highlight link TrailingSpaces Error
"    endif
"    match TrailingSpaces /\s\+$/
"endfunction
