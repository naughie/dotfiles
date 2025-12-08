return {
    -- https://lazy.folke.io/spec/examples
    {
        'Maya-sama/kawaii.nvim',
        lazy = false,
        priority = 1000,
        opts = {
            override_colors = {
                NormalFG = "#d5c3cb",
                NormalBG = "#2c2123",
            },
        },
        config = function(plugin, opts)
            require("kawaii").setup(opts)
            vim.cmd([[colorscheme kawaii]])
        end,
    },
}
