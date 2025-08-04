-- External tools
-- tree-sitter: cargo install --locked tree-sitter-cli
-- ts_ls: npm install -g typescript-language-server typescript
-- biome_ls: bun.js
return {
    -- https://lazy.folke.io/spec/examples
    {
        'Sonya-sama/kawaii.nvim',
        lazy = false,
        priority = 1000,
        config = function()
            vim.cmd([[colorscheme kawaii]])
            vim.api.nvim_set_hl(0, 'CursorLine', { link = 'Visual' })
        end,
    },
    -- {
    --     'sainnhe/everforest',
    --     lazy = false,
    --     priority = 1000,
    --     config = function()
    --         vim.opt.background = 'dark'
    --         vim.g.everforest_background = 'soft'
    --         vim.g.everforest_better_performance = 1
    --         vim.g.everforest_sign_column_background = 'gray'
    --         vim.g.everforest_diagnostic_text_highlight = 1
    --         vim.g.everforest_diagnostic_virtual_text = 'colored'
    --         vim.cmd([[colorscheme everforest]])
    --     end,
    -- },

    { "naughie/glocal-states.nvim", lazy = true },

    { "naughie/my-ui.nvim", lazy = true },

    {
        'naughie/lazy-filer.nvim',
        lazy = false,
        build = function(plugin)
            require('lazy-filer').build_and_spawn_filer(plugin.dir)
        end,
        opts = function(plugin)
            return {
                root_dir = plugin.dir,
            }
        end,
    },

    {
        'naughie/termplexer.nvim',
        lazy = false,
        opts = {
            open_term_if_no_file = true,
            dim = {
                width = function() return math.floor(vim.api.nvim_get_option('columns') * 0.5) end,
                height_output = function() return math.floor(vim.api.nvim_get_option('lines') * 0.8) end,
                height_input = 3,
            },

            keymaps = {
                global = {
                    { 'n', '<Space>t', 'open_or_create_term' },
                    { { 'n', 'i', 't' }, '<C-S-t>', function() vim.cmd('stopi | tabnew | vsplit | vsplit | Term') end },
                    { { 'n', 'i', 't' }, '<C-Tab>', function() vim.cmd('stopi | tabn') end },
                    { { 'n', 'i', 't' }, '<C-S-Tab>', function() vim.cmd('stopi | tabp') end },
                    { { 'n', 'i', 't' }, '<C-S-w>', function() vim.cmd('tabc') end },
                },

                input_buffer = {
                    { {  'n', 'i' }, '<CR>', 'send_cmd' },
                    { 'n', 'q', ':q<CR>' },
                    { { 'n', 'i' }, '<C-k>', 'move_to_output_win' },
                    { 'n', '<C-o>', 'open_file_from_input_buffer' },
                    { 'i', '<C-o>', function()
                        vim.cmd('stopi')
                        require("termplexer").fn.open_file_from_input_buffer()
                    end },

                    { 'n', 'k', 'cursor_up_or_history_prev' },
                    { 'n', 'j', 'cursor_down_or_history_next' },
                },

                output_buffer = {
                    { 'n', 'q', ':q<CR>' },
                    { 'n', 'i', 'open_cmdline_and_insert' },
                    { 'n', 'I', 'open_cmdline_and_insert' },
                    { 'n', 'a', 'open_cmdline_and_append' },
                    { 'n', 'A', 'open_cmdline_and_append' },
                    { 'n', 'o', 'open_file_under_cursor' },
                    { 'n', 'O', 'open_file_under_cursor' },
                    { 'v', 'o', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'v', 'O', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'v', '<CR>', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'n', '<C-j>', 'open_cmdline_and_move' },

                     { 'n', '<Space>i', 'enter_term_insert' },

                    { 't', '<C-q>', '<C-\\><C-n>' },
                    { 't', '<Esc>', '<C-\\><C-n>' },
                },
            },
        },
    },

    {
        'nvim-treesitter/nvim-treesitter',
        build = ':TSUpdate',
        lazy = false,
        branch = 'main',
        config = function () 
            local configs = require('nvim-treesitter')

            configs.install { 'bash', 'css', 'html', 'javascript', 'json', 'latex', 'lua', 'markdown', 'rust', 'toml', 'tsx', 'typescript', 'vim', 'vimdoc', 'yaml' }

            local treesitter_au = vim.api.nvim_create_augroup('nvim-treesitter-setup', { clear = true })
            vim.api.nvim_create_autocmd({ 'FileType' }, {
                pattern = { '*.css', '*.help', '*.html', '*.javascript', '*.javascriptreact', '*.json', '*.jsonc', '*.latex', '*.lua', '*.markdown', '*.plaintex', '*.rust', '*.sh', '*.toml', '*.typescript', '*.typescriptreact', '*.vim', '*.yaml' },
                group = treesitter_au,
                callback = function()
                    vim.treesitter.start()
                    vim.wo.foldexpr = 'v:lua.vim.treesitter.foldexpr()'
                    -- vim.bo.indentexpr = 'v:lua.require"nvim-treesitter".indentexpr()'
                end,
            })
        end
    },

    {
        'neovim/nvim-lspconfig',
        config = function()
            local configs = require('lsp-configs')
            require('lsp').setup(configs)
        end,
    },

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
