return {
    {
        'neovim/nvim-lspconfig',
        event = "VeryLazy",
        config = function()
            local configs = require('lsp-configs')
            require('lsp-setup').setup(configs)
        end,
    },
}
