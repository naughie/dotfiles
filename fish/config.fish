set -x LC_ALL en_US.UTF-8
set -x LANG en_US.UTF-8

set -x CPATH /usr/local/include $CPATH
set -x LIBRARY_PATH /usr/local/lib $LIBRARY_PATH
set -x LD_LIBRARY_PATH /usr/local/lib $LD_LIBRARY_PATH

# set EDITOR to either nvim or vim
command -v nvim >/dev/null &&
  set -x EDITOR nvim ||
  set -x EDITOR vim

# set config env
set -x XDG_CONFIG_HOME $HOME/.config

for file in $__fish_config_dir/paths/*.fish
    source $file
end

if status is-interactive
    for target in $setup_list
        "setup_$target"
    end

    for file in $__fish_config_dir/interactive/*.fish
        source $file
    end
else
    function setup
        set -l target "$argv[1]"
        switch "$target"
            case golang
                setup_go
            case nodejs
                setup_node
            case conda anaconda anaconda3 python3
                setup_python
            case rustc rustup cargo
                setup_rust
            case latex
                setup_tex
            case '*'
                "setup_$target"
        end
    end
end


fish_add_path $HOME/bin
