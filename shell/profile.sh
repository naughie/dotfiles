export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

_generated_profile_sh="$HOME/etc/dotfiles/shell/generated.profile.sh"
test -f "${_generated_profile_sh}" && . "${_generated_profile_sh}"

# Node
if [ -x "${FNM_DIR}/bin/fnm" ]; then
    eval "$("${FNM_DIR}/bin/fnm" env --shell bash)"
fi

# Hugging Face
export HF_HOME="${_TOOL_ROOT}/hf"

# TeX
export TEXMFLOCAL=/usr/local/texlive/texmf-local
export TEXMFHOME="${_TOOL_ROOT}/texmf"
_tex_year=2025
case "$(uname -ms)" in
    "Linux x86_64")
        _tex_dist=x86_64-linux
        ;;
    "Darwin arm64")
        _tex_dist=universal-darwin
        ;;
esac
export PATH="/usr/local/texlive/${_tex_year}/bin/${_tex_dist}:$PATH"


case "$-" in
    *i*)
        test -x "${_FISH_INSTALL}/fish" && exec "${_FISH_INSTALL}/fish" -il
        ;;
esac
