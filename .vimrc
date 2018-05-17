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

let g:python_host_prog  = $PYENV_ROOT . '/versions/neovim2/bin/python'
let g:python3_host_prog = $PYENV_ROOT . '/versions/neovim3/bin/python'
let g:deoplete#enable_at_startup = 1
let g:deoplete#auto_complete_delay = 0
let g:deoplete#auto_complete_start_length = 1

nnoremap <BS> <C-w>h
nnoremap <C-h> <C-w>h
set mouse-=a
set sh=zsh
"tnoremap <silent><ESC> <C-\><C-n>
"nnoremap ,tt :terminal<CR><C-\><C-n>:set nospell<CR>i
nnoremap ,ts :split<CR>:terminal<CR><C-\><C-n>:set nospell<CR>i
"let mapleader
nmap <Space>f :Denite file_rec<CR>

syntax on

filetype plugin indent on

set number
set ruler

set spelllang=en,cjk

set expandtab
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

set scrolloff=3

set display=lastline
set wrap

set cole=0

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

augroup colorSchemeSetting
  au!
  au ColorScheme * hi Normal ctermbg=none
augroup END

set cursorline
hi clear CursorLine

augroup QfGroup
  au!
  au QuickFixCmdPost *grep* cwindow
augroup END

augroup TrailingSpace
  au!
  au VimEnter,WinEnter,ColorScheme * highlight TrailingSpaces term=underline guibg=Red ctermbg=Red
  au VimEnter,WinEnter * match TrailingSpaces /\s\+$/
augroup END

augroup TwitVimSetting
  au!
  au FileType twitvim set wrap
augroup END

hi StatusLineFilename cterm=bold ctermfg=46
hi StatusLineCursorPosition ctermfg=184

nnoremap j gj
nnoremap k gk
nnoremap <Down> gj
nnoremap <Up> gk
nnoremap gj j
nnoremap gk k

nnoremap <silent><C-l> <C-w>l
nnoremap <silent><C-k> <C-w>k
nnoremap <silent><C-j> <C-w>j

nnoremap <Space>h ^
nnoremap <Space>l $
vnoremap <Space>h ^
vnoremap <Space>l $

nnoremap <Space>w :w<CR>
nnoremap <Space>q :q<CR>
nnoremap <Space>g :vim<Space>

vmap <Space>y "+y
nmap <Space>v <C-v>
nmap <Space>r :QuickRun<CR>

inoremap <C-p> <C-[>

nnoremap Y y$

nnoremap <C-p> "zdd<Up>"zP
nnoremap <C-n> "zdd"zp
vnoremap <C-p> "zx<Up>"zP`[V`]
vnoremap <C-n> "zx"zp`[V`]

nnoremap <silent><Space><Space> "zyiw:let @/ = '\<' . @z . '\>'<CR>:setlocal hlsearch<CR>
nnoremap <silent><Space>n :noh<CR>

nnoremap n nzz
nnoremap N Nzz
nnoremap * *zz
nnoremap # #zz
nnoremap g* g*zz
nnoremap g# g#zz

nmap <Space>s cs"'
nmap <Space>d cs'"

cnoremap <expr> / getcmdtype() == '/' ? '\/' : '/'
cnoremap <expr> ? getcmdtype() == '?' ? '\?' : '?'

imap <expr><C-k> pumvisible() ? "\<C-N>" : neosnippet#jumpable() ?  "\<Plug>(neosnippet_expand_or_jump)" : "\<TAB>"
smap <expr><C-k> neosnippet#jumpable() ?  "\<Plug>(neosnippet_expand_or_jump)" : "\<TAB>"
inoremap <expr><C-K>  pumvisible() ? "\<C-p>" : "\<S-TAB>"

nnoremap <Space>m :!make<CR>

nnoremap t <Nop>
nnoremap tt <C-]>
nnoremap tj :tag<CR>
nnoremap tk :pop<CR>
nnoremap tl :tags<CR>

nnoremap ; :
nnoremap : ;

nnoremap <silent><Space>r :QuickRun<CR>

let g:tex_conceal = ''

let g:markdown_syntax_conceal = ''

let g:vim_markdown_folding_disabled=1

let g:unite_enable_start_insert=1

augroup TeXMakePDFRealTime
  autocmd!
  autocmd BufWritePost,FilterWritePost,FileAppendPost,FileWritePost {root\|pref\|packages\|commands}\@!*.tex QuickRun -type tex_one
  autocmd VimLeave *.tex QuickRun -type tex_remove
augroup END

augroup MarkdownCtagsUpdate
  autocmd!
  autocmd BufWritePost,FilterWritePost,FileAppendPost,FileWritePost /Users/nakatam/markdowns/*.md QuickRun -type ctags
augroup END

augroup CppCtagsUpdate
  autocmd!
  "autocmd BufWritePost,FilterWritePost,FileAppendPost,FileWritePost {/Users/nakatam/}\@!*.{cpp\|h} QuickRun -type ctags
augroup END

let g:opamshare = substitute(system('opam config var share'),'\n$','','''')
execute 'set rtp+=' . g:opamshare . '/merlin/vim'
execute 'set rtp^=' . g:opamshare . '/ocp-indent/vim'

function! s:ocaml_format()
    let now_line = line('.')
    exec ':%! ocp-indent'
    exec ':' . now_line
endfunction

augroup ocaml_format
    autocmd!
    autocmd BufWrite,FileWritePre,FileAppendPre *.mli\= call s:ocaml_format()
augroup END
