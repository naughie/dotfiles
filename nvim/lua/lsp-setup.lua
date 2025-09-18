local lsp = {
    "rust_analyzer",
    "ts_ls",
    "denols",
    "biome",
    "texlab",
}

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

local function common_on_attach()
    local mygroup = vim.api.nvim_create_augroup('MyLspAttachCommon', { clear = true })
    vim.api.nvim_create_autocmd('LspAttach', {
        group = mygroup,
        pattern = '*',
        callback = function(ev)
            local client = vim.lsp.get_client_by_id(ev.data.client_id)
            local bufnr = ev.buf

            enable_inlayhints(client, bufnr)
            define_keymaps(bufnr)
        end,
    })
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

    local diag_win = nil
    local diag_buf = nil

    local mygroup = vim.api.nvim_create_augroup('MyOpenDiagnosticAuto', { clear = true })
    vim.api.nvim_create_autocmd('CursorHold', {
        group = mygroup,
        pattern = '*',
        callback = function()
            if is_float_open() then return end
            local diag = vim.diagnostic.get(0, { severity = { min = vim.diagnostic.severity.WARN } })
            if #diag > 0 then
                if diag_win and vim.api.nvim_win_is_valid(diag_win) then
                    vim.api.nvim_win_close(diag_win, true)
                end
                if diag_buf and vim.api.nvim_buf_is_valid(diag_buf) then
                    vim.api.nvim_buf_delete(diag_buf, { force = true })
                end

                local buf, win = vim.diagnostic.open_float({ border = 'rounded', scope = 'cursor' })
                diag_win = win
                diag_buf = buf
            end
        end,
    })
    vim.api.nvim_create_autocmd('WinLeave', {
        group = mygroup,
        pattern = '*',
        callback = function()
            if diag_win and vim.api.nvim_win_is_valid(diag_win) then
                vim.api.nvim_win_close(diag_win, true)
            end
            if diag_buf and vim.api.nvim_buf_is_valid(diag_buf) then
                vim.api.nvim_buf_delete(diag_buf, { force = true })
            end
        end,
    })
end

local function fmt_on_save()
    local mygroup = vim.api.nvim_create_augroup('MyFmtOnSave', { clear = true })
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

return {
    setup = function()
        common_on_attach()
        diag_on_hold()
        fmt_on_save()

        for _, key in ipairs(lsp) do
            vim.lsp.enable(key)
        end
    end,
}
