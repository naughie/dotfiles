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
}
