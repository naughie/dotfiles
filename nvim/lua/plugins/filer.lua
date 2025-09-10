return {
    {
        'naughie/lazy-filer.nvim',
        lazy = false,
        opts = function(plugin)
            return {
                root_dir = plugin.dir,
                border = {
                    hl_group = "Keyword",
                },
                rpc_ns = "lazy-filer",

                hl = {
                    directory = { link = "Operator" },
                },

                keymaps = {
                    global = {
                        { 'n', '<C-e>', 'new_filer' },
                    },

                    filer = {
                        { 'n', 'o', 'open_or_expand' },
                        { 'n', '<CR>', 'open_or_expand' },
                        { 'n', 'c', 'chdir_to_cursor' },
                        { 'n', 'u', 'move_to_parent' },
                        { 'n', '<C-n>', 'open_new_entry_win' },
                        { 'n', 'd', 'open_delete_entry_win' },
                        { 'n', 'r', 'refresh' },
                        { 'n', 'm', 'open_rename_entry_win' },
                        { 'n', 'q', 'close_filer' },

                        { { 'n', 'i' }, '<C-j>', 'move_to_subwin' },
                    },

                    new_entry = {
                        { { 'n', 'i' }, '<CR>', 'create_entry' },
                        { 'n', 'q', 'close_subwin' },

                        { { 'n', 'i' }, '<C-k>', 'move_to_filer' },
                    },

                    rename_entry = {
                        { { 'n', 'i' }, '<CR>', 'rename_entry' },
                        { 'n', 'q', 'close_subwin' },

                        { { 'n', 'i' }, '<C-k>', 'move_to_filer' },
                    },
                },
            }
        end,
    },
}
