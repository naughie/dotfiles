return {
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
}
