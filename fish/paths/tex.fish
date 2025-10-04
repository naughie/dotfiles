function setup_tex
    if test -d "/usr/local/texlive/$TEX_YEAR"
        # set -x MANPATH "/usr/local/texlive/$TEX_YEAR/texmf-dist/doc/man:$MANPATH"
        # set -x INFOPATH /usr/local/texlive/$TEX_YEAR/texmf-dist/doc/info $INFOPATH
        fish_add_path "/usr/local/texlive/$TEX_YEAR/bin/$TEX_DIST"
        set -x TEXMFLOCAL /usr/local/texlive/texmf-local
        set -x TEXMFHOME "$HOME/etc/texmf"
    end
end
set -ax setup_list tex
