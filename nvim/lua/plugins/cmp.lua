return {
    {
        'saghen/blink.cmp',
        event = { 'InsertEnter', 'CmdLineEnter' },

        version = '1.*',

        opts = {
            keymap = {
                preset = 'none',
                ['<C-e>'] = { 'cancel' },
                ['<S-Tab>'] = { 'select_prev' },
                ['<Tab>'] = { 'select_next', 'fallback' },
                ['<CR>'] = { 'accept', 'fallback' },
                -- ['<C-k>'] = { 'show_documentation', 'hide_documentation' },
                -- ['<C-k>'] = { 'show_signature', 'hide_signature' },
            },

            completion = {
                documentation = {
                    auto_show = true,
                    auto_show_delay_ms = 300,
                    window = { border = 'rounded' },
                },
                list = {
                    cycle = { from_top = true },
                    selection = { preselect = false },
                },
                menu = { border = 'rounded' },
            },

            sources = {
                default = { 'lsp', 'path', 'snippets', 'buffer' },
                providers = {
                    path = {
                        opts = {
                            get_cwd = function(_)
                                return vim.fn.getcwd()
                            end,
                            trailing_slash = false,
                            show_hidden_files_by_default = true,
                        },
                    },
                },
            },

            fuzzy = {
                implementation = 'prefer_rust',
                max_typos = 0,
                use_frecency = false,
                sorts = {
                    'exact',
                    'score',
                    'sort_text',
                },
            },

            signature = { enabled = true },

            cmdline = {
                completion = {
                    list = {
                        selection = { preselect = false },
                    },
                },
            },
        },
        opts_extend = { 'sources.default' },
    },
}
