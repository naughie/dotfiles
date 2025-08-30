return {
    {
        'naughie/termplexer.nvim',
        lazy = false,
        opts = {
            open_term_if_no_file = false,
            dim = {
                width = function() return math.floor(vim.api.nvim_get_option('columns') * 0.5) end,
                height_output = function() return math.floor(vim.api.nvim_get_option('lines') * 0.8) end,
                height_input = 3,
            },
            border = {
                hl_group = "Keyword",
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
                    { 'n', 'q', 'close_win' },
                    { { 'n', 'i' }, '<C-k>', 'move_to_output_win' },
                    { 'n', '<C-o>', 'open_file_from_input_buffer' },
                    { 'i', '<C-o>', function()
                        vim.cmd('stopi')
                        require("termplexer").fn.open_file_from_input_buffer()
                    end },

                    { 'n', '<Space>i', 'enter_term_insert' },
                    { 'n', 'I', 'enter_term_insert' },

                    { 'n', 'k', 'cursor_up_or_history_prev' },
                    { 'n', 'j', 'cursor_down_or_history_next' },
                    { { 'n', 'i' }, '<C-c>', 'send_sigint' },
                },

                output_buffer = {
                    { 'n', 'q', 'close_win' },
                    { 'n', 'i', 'open_cmdline_and_insert' },
                    { 'n', 'a', 'open_cmdline_and_append' },
                    { 'n', 'A', 'open_cmdline_and_append' },
                    { 'n', 'o', 'open_file_under_cursor' },
                    { 'n', 'O', 'open_file_under_cursor' },
                    { 'v', 'o', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'v', 'O', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'v', '<CR>', ':<C-u>lua require("termplexer").fn.open_file_from_selection()<CR>' },
                    { 'n', '<C-j>', 'open_cmdline_and_move' },

                    { 'n', '<C-c>', 'send_sigint' },

                    { 'n', '<Space>i', 'enter_term_insert' },
                    { 'n', 'I', 'enter_term_insert' },

                    { 't', '<C-q>', '<C-\\><C-n>' },
                    { 't', '<Esc>', '<Esc><C-\\><C-n>' },
                },
            },
        },
    },
}
