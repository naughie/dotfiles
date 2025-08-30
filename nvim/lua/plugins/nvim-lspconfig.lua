return {
    {
        'neovim/nvim-lspconfig',
        config = function()
            local configs = require('lsp-configs')
            require('lsp-setup').setup(configs)
        end,
    },
}
