local M = {}

local function define_keymaps(bufnr)
    local keymap_opts = { buffer = bufnr, silent = true }
    vim.keymap.set('n', 'gD', vim.lsp.buf.declaration, keymap_opts)
    vim.keymap.set('n', 'gd', vim.lsp.buf.definition, keymap_opts)
    vim.keymap.set('n', 'K', function() vim.lsp.buf.hover({ border = 'rounded' }) end, keymap_opts)
    vim.keymap.set('n', '<Space>r', vim.lsp.buf.rename, keymap_opts)
end

local function enable_inlayhints(client, bufnr)
    if client:supports_method('textDocument/inlayHint') then
        vim.lsp.inlay_hint.enable(true, { bufnr = bufnr })
    end
end

local function common_on_attach(client, bufnr)
    enable_inlayhints(client, bufnr)
    define_keymaps(bufnr)
end

local function diag_on_hold()
    local function is_float_open()
        for _, win in ipairs(vim.api.nvim_list_wins()) do
            if vim.api.nvim_win_get_config(win).relative ~= '' then
                return true
            end
        end
        return false
    end

    local mygroup = vim.api.nvim_create_augroup('OpenDiagnosticAuto', { clear = true })
    vim.api.nvim_create_autocmd('CursorHold', {
        group = mygroup,
        pattern = '*',
        callback = function()
            if is_float_open() then return end
            local diag = vim.diagnostic.get(0, { severity = { min = vim.diagnostic.severity.WARN } })
            if #diag > 0 then
                vim.diagnostic.open_float({ border = 'rounded', scope = 'cursor' })
            end
        end,
    })
end

local function fmt_on_save()
    local mygroup = vim.api.nvim_create_augroup('FmtOnSave', { clear = true })
    vim.api.nvim_create_autocmd('BufWritePre', {
        group = mygroup,
        pattern = '*',
        callback = function(args)
            local fmt_clients = vim.lsp.get_clients({
                bufnr = args.buf,
                method = 'textDocument/formatting',
            })

            if #fmt_clients > 0 then
                vim.lsp.buf.format({ bufnr = args.buf, async = false, timeout_ms = 500 })
            end
        end,
    })
end

function M.setup(configs)
    for key, value in pairs(configs) do
        local config = vim.deepcopy(value)
        if value.on_attach then
            config.on_attach = function(client, bufnr)
                value.on_attach(client, bufnr)
                common_on_attach(client, bufnr)
            end
        else
            config.on_attach = common_on_attach
        end

        vim.lsp.config(key, config)
        vim.lsp.enable(key)
    end

    diag_on_hold()
    fmt_on_save()
end

return M
