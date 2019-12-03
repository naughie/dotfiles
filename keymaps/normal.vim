nnoremap <BS> <C-w>h
nnoremap <C-h> <C-w>h

"tnoremap <silent><ESC> <C-\><C-n>
"nnoremap ,tt :terminal<CR><C-\><C-n>:set nospell<CR>i
nnoremap ,ts :split<CR>:terminal<CR><C-\><C-n>:set nospell<CR>i
"let mapleader

nnoremap <silent>j gj
nnoremap <silent>k gk
nnoremap <silent><Down> gj
nnoremap <silent><Up> gk
nnoremap <silent>gj j
nnoremap <silent>gk k

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

nnoremap Y y$
" nnoremap <CR> o<ESC>

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

nnoremap <silent><Space>r :QuickRun<CR>

nnoremap <C-t> :Template<Space>

nmap <Up> <C-p>
nmap <Down> <C-n>
nmap <Right> <C-f>
nmap <Left> <C-b>
nmap <Home> <C-a>
nmap <End> <C-e>

nnoremap <silent><Space><CR> A<CR><ESC>

command TODO vim TODO **
