__my_setup_tools_4d75232a() {
    local __generated_profile_sh="$HOME/etc/dotfiles/shell/generated.profile.sh"
    test -f "${__generated_profile_sh}" || return
    . "${__generated_profile_sh}"

    test -x "${_my_fish_install}/fish" && _fish_exec_4d75232a="${_my_fish_install}/fish"

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

    _my_clear_generated_envs

    # Node
    if [ -x "${FNM_DIR}/bin/fnm" ]; then
        eval "$("${FNM_DIR}/bin/fnm" env --shell bash)"
    fi

    unset -f __my_setup_tools_4d75232a
}


export LC_ALL=en_US.UTF-8
export LANG=en_US.UTF-8

export C_INCLUDE_PATH="/usr/local/include:${C_INCLUDE_PATH}"
export CPLUS_INCLUDE_PATH="/usr/local/include:${CPLUS_INCLUDE_PATH}"
export LIBRARY_PATH="/usr/local/lib:${LIBRARY_PATH}"
export LD_LIBRARY_PATH="/usr/local/lib:${LD_LIBRARY_PATH}"

__my_setup_tools_4d75232a

case "$-" in
    *i*)
        test -n "${_fish_exec_4d75232a}" && exec "${_fish_exec_4d75232a}" -il
        ;;
esac
