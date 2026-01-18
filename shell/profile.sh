__my_load_generated_profile() {
    local __generated_profile_sh="$HOME/etc/dotfiles/shell/generated.profile.sh"
    test -f "${__generated_profile_sh}" && . "${__generated_profile_sh}"
}

__my_setup_tools() {
    # Node
    if [ -x "${FNM_DIR}/bin/fnm" ]; then
        eval "$("${FNM_DIR}/bin/fnm" env --shell bash)"
    fi

    # Hugging Face
    export HF_HOME="${_my_tool_root}/hf"

    # TeX
    export TEXMFLOCAL=/usr/local/texlive/texmf-local
    export TEXMFHOME="${_my_tool_root}/texmf"
    local __my_tex_year=2025
    case "$(uname -ms)" in
        "Linux x86_64")
            local __my_tex_dist=x86_64-linux
            ;;
        "Darwin arm64")
            local __my_tex_dist=universal-darwin
            ;;
    esac
    export PATH="/usr/local/texlive/${__my_tex_year}/bin/${__my_tex_dist}:$PATH"
}


export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export CPATH=/usr/local/include:$CPATH
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

__my_load_generated_profile
__my_setup_tools

case "$-" in
    *i*)
        test -x "${_my_fish_install}/fish" && exec "${_my_fish_install}/fish" -il
        ;;
esac
