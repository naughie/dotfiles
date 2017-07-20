if has('nvim')
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

nnoremap <silent><BS> <C-w>h
set mouse-=a
set sh=zsh
tnoremap <silent><ESC> <C-\><C-n>
nnoremap ,tt :terminal<CR><C-\><C-n>:set nospell<CR>i
nnoremap ,ts :split<CR>:terminal<CR><C-\><C-n>:set nospell<CR>i
let mapleader = "\<Space>"
nmap <leader>f :Denite file_rec<CR>

elseif 1

if 0 | endif

if has('vim_starting')
  if &compatible
    set nocompatible
  endif
  set runtimepath+=~/.vim/bundle/neobundle.vim/
endif

call neobundle#begin(expand('~/.vim/bundle/'))

NeoBundleFetch 'Shougo/neobundle.vim'

NeoBundle 'thinca/vim-quickrun'

NeoBundle 'Shougo/vimproc',{
            \'build' : {
            \  'windows' : 'make -f make_mingw32.mak',
            \  'cygwin' : 'make -f make_cygwin.mak',
            \  'mac' : 'make -f make_mac.mak',
            \  'unix' : 'make -f make_unix.mak',
            \  },
            \ }

NeoBundle 'Shougo/unite.vim'

NeoBundle 'osyo-manga/unite-quickfix'

NeoBundle 'osyo-manga/shabadou.vim'

"NeoBundle 'lervag/vimtex'

NeoBundle 'tomasr/molokai'

NeoBundle 'Shougo/neosnippet'

NeoBundle 'Shougo/neocomplete'

NeoBundle 'Shougo/neosnippet-snippets'

NeoBundle 'nelstrom/vim-visual-star-search'

NeoBundle 'tpope/vim-fugitive'

NeoBundle 'vim-ruby/vim-ruby'

NeoBundle 'tpope/vim-endwise'

NeoBundle 'plasticboy/vim-markdown'

NeoBundle 'kannokanno/previm'

NeoBundle 'tyru/open-browser.vim'

NeoBundle 'derekwyatt/vim-scala'

NeoBundle 'scrooloose/nerdtree'

"NeoBundle 'kana/vim-filetype-haskell'

NeoBundle 'itchyny/vim-haskell-indent'

NeoBundle 'eagletmt/ghcmod-vim'

NeoBundle 'ujihisa/neco-ghc'

NeoBundle 'osyo-manga/vim-watchdogs'

NeoBundle 'thinca/vim-ref'

NeoBundle 'ujihisa/ref-hoogle'

NeoBundle 'tpope/vim-surround'

call neobundle#end()



NeoBundleCheck

noremap ,ub :Unite buffer<CR>
noremap ,uu :Unite -buffer-name=file file<CR>
nnoremap ,un :Unite file/new<CR>
augroup UniteGroup
  au FileType unite nnoremap <silent> <buffer> <ESC><ESC> :q<CR> |
                  \ inoremap <silent> <buffer> <ESC><ESC> <ESC>:q<CR>
augroup END

nnoremap <silent><C-h> <C-w>h
let mapleader = "\<Space>"
endif

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
set statusline+=%F
set statusline+=%{fugitive#statusline()}

set noswapfile

set vb t_vb=

augroup colorSchemeSetting
  au!
  au ColorScheme * hi Normal ctermbg=none |
                 \ hi LineNr ctermbg=none
augroup END

colorscheme molokai

set cursorline
hi clear CursorLine

augroup htmlMarkdownGroup
  au!
  au FileType markdown hi htmlItalic ctermfg=3 |
                     \ hi htmlBold ctermfg=9 |
                     \ hi link htmlLink Comment |
                     \ set spell
  au FileType html hi htmlItalic ctermfg=3 |
                 \ hi htmlBold ctermfg=9 |
                 \ hi link htmlLink Comment |
                 \ set spell
augroup END

augroup texGroup
  au!
  au FileType tex hi texBoldStyle ctermfg=9 |
                \ hi texItalStyle ctermfg=3 |
                \ hi texBoldItalStyle ctermfg=3 |
                \ hi texItalBoldStyle ctermfg=9 |
                \ set spell |
                \ filetype indent off |
                \ set noautoindent |
                \ set nosmartindent |
augroup END

augroup QfGroup
  au!
  au QuickFixCmdPost *grep* cwindow
augroup END

augroup RubyGroup
  au!
  au FileType ruby hi rubySymbol cterm=NONE ctermfg=135 |
                 \ hi rubyInteger ctermfg=10 |
                 \ hi rubyFloat ctermfg=10 |
                 \ hi rubyBoolean ctermfg=10 |
                 \ hi rubyPseudoVariable cterm=bold ctermfg=10
augroup END

augroup ERubyGroup
  au!
  au FileType eruby hi htmlItalic ctermfg=3 |
                  \ hi htmlBold ctermfg=9 |
                  \ hi htmlLink cterm=NONE ctermfg=38 |
                  \ hi rubySymbol cterm=NONE ctermfg=135 |
                  \ hi rubyInteger ctermfg=10 |
                  \ hi rubyFloat ctermfg=10 |
                  \ hi rubyBoolean ctermfg=10 |
                  \ hi rubyPseudoVariable cterm=bold ctermfg=10
augroup END

augroup TrailingSpace
  au!
  au VimEnter,WinEnter,ColorScheme * highlight TrailingSpaces term=underline guibg=Red ctermbg=Red
  au VimEnter,WinEnter * match TrailingSpaces /\s\+$/
augroup END

hi Todo ctermbg=NONE
hi Visual ctermfg=white ctermbg=darkgray

nnoremap j gj
nnoremap k gk
nnoremap <Down> gj
nnoremap <Up> gk
nnoremap gj j
nnoremap gk k

nnoremap <silent><C-l> <C-w>l
nnoremap <silent><C-k> <C-w>k
nnoremap <silent><C-j> <C-w>j

nnoremap <leader>h ^
nnoremap <leader>l $
vnoremap <leader>h ^
vnoremap <leader>l $

inoremap <C-j> <BS>
cnoremap <C-j> <BS>

nnoremap <leader>w :w<CR>
nnoremap <leader>q :q<CR>
nnoremap <leader>g :vim<Space>
nmap <leader>s cs"'
nmap <leader>d cs'"
vmap <leader>y "+y
nmap <leader>v <C-v>
nmap <leader>r :QuickRun<CR>

inoremap <C-p> <C-[>

nnoremap Y y$

nnoremap <C-p> "zdd<Up>"zP
nnoremap <C-n> "zdd"zp
vnoremap <C-p> "zx<Up>"zP`[V`]
vnoremap <C-n> "zx"zp`[V`]

nnoremap <silent><leader><leader> "zyiw:let @/ = '\<' . @z . '\>'<CR>:set hlsearch<CR>
nnoremap <silent><leader>n :noh<CR>

nnoremap n nzz
nnoremap N Nzz
nnoremap * *zz
nnoremap # #zz
nnoremap g* g*zz
nnoremap g# g#zz

cnoremap <expr> / getcmdtype() == '/' ? '\/' : '/'
cnoremap <expr> ? getcmdtype() == '?' ? '\?' : '?'

"nnoremap <leader>t :GhcModType<CR>
"cnoremap cmod GhcModTypeClear

"if has('conceal')
"    set conceallevel=0 concealcursor=
"endif
"set conceallevel=0

let g:tex_conceal = ''

let g:markdown_syntax_conceal = ''

let g:vim_markdown_folding_disabled=1

let g:unite_enable_start_insert=1
