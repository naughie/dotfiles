function setup_node
    set FNM_PATH $FNM_DIR/bin
    if [ -d "$FNM_PATH" ]
        set PATH "$FNM_PATH" $PATH
        fnm env --shell fish | source
    end
end
set -ax setup_list node

function setup_deno
    if [ -d "$DENO_INSTALL" ]
        fish_add_path "$DENO_INSTALL/bin"
    end
end
set -ax setup_list deno

function setup_bun
    if [ -d "$BUN_INSTALL" ]
        fish_add_path "$BUN_INSTALL/bin"
    end
end
set -ax setup_list bun
