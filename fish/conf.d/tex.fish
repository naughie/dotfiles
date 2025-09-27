# tex
set TEX_YEAR 2025
set TEX_DIST x86_64-linux
# set TEX_DIST universal-darwin
# set -x MANPATH "/usr/local/texlive/$TEX_YEAR/texmf-dist/doc/man:$MANPATH"
# set -x INFOPATH /usr/local/texlive/$TEX_YEAR/texmf-dist/doc/info $INFOPATH
fish_add_path /usr/local/texlive/$TEX_YEAR/bin/$TEX_DIST
set -x TEXMFLOCAL /usr/local/texlive/texmf-local
set -x TEXMFHOME $HOME/etc/texmf
