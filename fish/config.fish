set -x LC_ALL en_US.UTF-8
set -x LANG en_US.UTF-8

set -x CPATH /usr/local/include $CPATH
set -x LIBRARY_PATH /usr/local/lib $LIBRARY_PATH
set -x LD_LIBRARY_PATH /usr/local/lib $LD_LIBRARY_PATH

# set config env
set -x XDG_CONFIG_HOME $HOME/.config

if status is-interactive
    for file in $__fish_config_dir/paths/*.fish
        source $file
    end

    for file in $__fish_config_dir/interactive/*.fish
        source $file
    end
else
    function setup_impl
        set -l target $argv[1]
        source "$__fish_config_dir/paths/$target.fish"
    end
    function setup
        set -l target "$argv[1]"
        switch "$target"
            case golang
                setup_impl go
            case nodejs
                setup_impl node
            case conda anaconda anaconda3 python3
                setup_impl python
            case rustc rustup cargo
                setup_impl rust
            case latex
                setup_impl tex
            case '*'
                setup_impl $target
        end
    end

    function nvim_scratch
        for file in $__fish_config_dir/paths/*.fish
            source $file
        end
        source $__fish_config_dir/interactive/term.fish

        nvim $argv
    end
end


fish_add_path $HOME/bin

command -v nvim >/dev/null &&
  set -x EDITOR nvim ||
  set -x EDITOR vim
