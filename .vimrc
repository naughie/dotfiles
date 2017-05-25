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
nnoremap ,df :Denite file_rec<CR>
nnoremap ,dr :Denite -resume<CR>
set mouse-=a
set sh=zsh
tnoremap <silent><ESC> <C-\><C-n>
nnoremap ,tt :terminal<CR><C-\><C-n>:set nospell<CR>i
nnoremap ,ts :split<CR>:terminal<CR><C-\><C-n>:set nospell<CR>i


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

"Plugins with NeoBundle
"NeoBundle 'hoge/fuga'

"NeoBundle 'Townk/vim-autoclose'

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

filetype plugin indent on


NeoBundleCheck

noremap ,ub :Unite buffer<CR>
noremap ,uu :Unite -buffer-name=file file<CR>
nnoremap ,un :Unite file/new<CR>
au FileType unite nnoremap <silent> <buffer> <ESC><ESC> :q<CR>
au FileType unite inoremap <silent> <buffer> <ESC><ESC> <ESC>:q<CR>

nnoremap <silent><C-h> <C-w>h
endif

syntax on




"autocmd BufRead,BufNewFile *.md
"setfiletype mkd

"set a function in vim
"set hoge; set hoge=fuga
set number

set spelllang=en,cjk

set spell

set expandtab

set shiftwidth=2

set softtabstop=2

set autoindent

set smartindent

set pumheight=10

set showmatch

set matchtime=1

set wrap

set ignorecase

set smartcase

set scrolloff=3

set ruler

set display=lastline

set cole=0

set backspace=indent,eol,start

set laststatus=2

set statusline+=%F
set statusline+=%{fugitive#statusline()}

set nohlsearch

"set background similarly to iTerm
autocmd ColorScheme * highlight Normal ctermbg=none

autocmd ColorScheme * highlight LineNr ctermbg=none

colorscheme molokai

au BufRead,BufNewFile *.md set filetype=markdown
au BufRead,BufNewfile *.tex set filetype=tex
au BufRead,BufNewfile *.toml set filetype=vim
au Filetype qf set nospell

au FileType * setl cole=0

autocmd! FileType markdown hi! def link markdownItalic LineNr

autocmd QuickFixCmdPost *grep* cwindow

"map a key binding to another
"[n/i/s/x][nore]map before after
"a-zA-Z0-9 -> unchanged
"cursor -> <Up><Down><Right><Left>
"control-* -> <C-*>
"space -> <Space>
"tab -> <Tab>
"escape -> <Esc>
"carriage return -> <CR>
"line feed -> <LF>
nnoremap j gj
nnoremap k gk
nnoremap <Down> gj
nnoremap <Up> gk
nnoremap gj j
nnoremap gk k


nnoremap <silent><C-l> <C-w>l
nnoremap <silent><C-k> <C-w>k
nnoremap <silent><C-j> <C-w>j
nnoremap <leader>s     <C-w>x

nnoremap <silent><C-e> :NERDTreeToggle<CR>

nnoremap <Space>h ^
nnoremap <Space>l $

"inoremap jk  <Esc>

inoremap <C-j> <BS>
cnoremap <C-j> <BS>

imap <C-k>  <Plug>(neosnippet_expand_or_jump)
smap <C-k>  <Plug>(neosnippet_expand_or_jump)
xmap <C-k>  <Plug>(neosnippet_expand_target)

imap <expr><TAB> neosnippet#expandable_or_jumpable() ?
            \ "\<Plug>(neosnippet_expand_or_jump)"
            \: pumvisible() ? "\<C-n>" : "\<TAB>"

smap <expr><TAB> neosnippet#expandable_or_jumpable() ?
            \ "\<Plug>(neosnippet_expand_or_jump)"
            \: "\<TAB>"

"nnoremap <leader>t :GhcModType<CR>
"cnoremap cmod GhcModTypeClear


"if has('conceal')
"    set conceallevel=0 concealcursor=
"endif
"set conceallevel=0

"configuration of a plugin
"let hoge = { 'foo' : 'bar' }
let g:neosnippet#snippets_directory='~/dotfiles/snippets/'

let g:tex_conceal = ''

let g:markdown_syntax_conceal = ''

let g:quickrun_config = {
            \ 'split' : '',
			\ 'runner' : 'vimproc',
			\ 'runner/vimproc/updatetime' : 10,
                        \ 'markdown' : {
                        \ 'outputter' : 'browser',
                        \ },
			\ 'tex' : {
			\ 'command' : 'lat',
			\ 'cmdopt' : '',
			\ 'exec' : ['%c %o %s'] 
			\ },
                        \ 'haskell' : {
                        \ 'command' : 'stack',
                        \ 'cmdopt' : 'runhaskell',
                        \ 'exec' : ['%c %o %s']
                        \ },
			\}

let g:vim_markdown_folding_disabled=1

let g:unite_enable_start_insert=1
