return {
    {
        "naughie/rg-fancy.nvim",
        event = "VeryLazy",
        opts = function(plugin)
            return {
                plugin_dir = plugin.dir,
                rpc_ns = "rg-fancy",
                border = {
                    hl_group = "Keyword",
                },
                hl = {
                    input_hint = { link = "Label" },
                    path = { link = "Operator" },
                    matched = { link = "Visual" },
                    header = { link = "Normal" },
                },

                context_length = 3,

                keymaps = {
                    global = {
                        { 'n', '<C-g>', 'open_results' },
                    },

                    results = {
                        { 'n', 'q', 'close_results' },
                        { 'n', 'i', 'open_and_ins_input' },
                        { 'n', '<C-j>', 'focus_input' },

                        { 'n', '<CR>', 'open_item_current' },
                        { 'n', 'o', 'open_item_current' },
                        { 'n', 'O', 'open_item_current' },

                        { 'n', 'j', 'goto_next_item_line' },
                        { 'n', 'k', 'goto_prev_item_line' },
                        { 'n', 'gg', 'goto_first_item_line' },
                        { 'n', 'G', 'goto_last_item_line' },
                    },

                    input = {
                        { 'n', 'q', 'close_input' },
                        { 'n', '<C-k>', 'focus_results' },
                        { 'n', 'd', 'clear_input' },
                        { 'n', '<CR>', 'grep' },
                    },
                },
            }
        end,
    },
}
