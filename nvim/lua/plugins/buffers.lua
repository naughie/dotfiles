return {
    {
        "naughie/buffer-switcher.nvim",
        event = "VeryLazy",
        opts = function(plugin)
                return {
                    plugin_dir = plugin.dir,
                    rpc_ns = "buffer-switcher",
                    border = {
                        hl_group = "Keyword",
                    },
                    hl = {
                        frame_title = { link = "Operator" },
                    },

                    keymaps = {
                        global = {
                            { "n", "<C-b>", "open" },
                        },

                        input = {
                            { "i", "<ESC>", "close" },
                            { "i", "<CR>", "open_selected_buf" },

                            { "i", "<Tab>", "select_next" },
                            { "i", "<Down>", "select_next" },
                            { "i", "<S-Tab>", "select_prev" },
                            { "i", "<Up>", "select_prev" },
                        },
                    },
                }
        end,
    },
}
