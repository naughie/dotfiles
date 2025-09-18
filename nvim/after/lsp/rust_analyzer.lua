return {
    cmd = (function()
        local path = vim.fn.expand('~/bin/rust-analyzer')

        if vim.fn.executable(path) == 1 then
          return { path }
        else
          return { 'rustup', 'run', 'stable', 'rust-analyzer' }
        end
    end)(),
    settings = {
        -- https://rust-analyzer.github.io/book/configuration
        ['rust-analyzer'] = {
            imports = {
                granularity = {
                    group = 'module',
                },
                prefix = 'self',
            },
            cargo = {
                buildScripts = {
                    enable = true,
                },
                features = 'all',
            },
            check = {
                command = 'clippy',
            },
            procMacro = {
                enable = true,
            },
            completion = {
                callable = {
                    snippets = 'none',
                },
            },
        },
    },
}
