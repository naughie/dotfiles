local function file_exists(path)
    local stat = vim.uv.fs_stat(path, nil)
    return stat ~= nil
end

local function find_js_project_root(bufnr)
    local current_buf_path = vim.api.nvim_buf_get_name(bufnr)

    if current_buf_path == '' then
        return nil
    end

    local current_dir = vim.fs.dirname(current_buf_path)

    while true do
        if file_exists(current_dir .. '/deno.json') or file_exists(current_dir .. '/deno.jsonc') then
            return {
                root_dir = current_dir,
                type = 'deno',
            }
        end

        if file_exists(current_dir .. '/package.json') then
            return {
                root_dir = current_dir,
                type = 'node',
            }
        end

        local parent_dir = vim.fs.dirname(current_dir)

        -- if current_dir == '/'
        if parent_dir == current_dir then
            break
        end

        current_dir = parent_dir
    end

    return nil
end

return {
    rust_analyzer = {
        cmd = { 'rustup', 'run', 'stable', 'rust-analyzer' },
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
    },

    ts_ls = {
        on_attach = function(client, bufnr)
            client.server_capabilities.documentFormattingProvider = false
            client.server_capabilities.documentRangeFormattingProvider = false
        end,
        root_dir = function(bufnr, on_dir)
            local root = find_js_project_root(bufnr)
            if root and root.type == 'node' then
                on_dir(root.root_dir)
            end
        end,
        settings = {
            typescript = {
                inlayHints = {
                    includeInlayParameterNameHints = 'all',
                    includeInlayParameterNameHintsWhenArgumentMatchesName = false,
                    includeInlayFunctionParameterTypeHints = true,
                    includeInlayVariableTypeHints = true,
                    includeInlayFunctionLikeReturnTypeHints = true,
                    includeInlayEnumMemberValueHints = true,
                },
            },
        },
    },

    denols = {
        root_dir = function(bufnr, on_dir)
            local root = find_js_project_root(bufnr)
            if root and root.type == 'deno' then
                on_dir(root.root_dir)
            end
        end,
        settings = {
            -- https://github.com/denoland/vscode_deno
            deno = {
                lint = true,
                inlayHints = {
                    parameterNames = { enabled = 'all', suppressWhenArgumentMatchesName = true },
                    variableTypes = { enabled = true },
                    parameterTypes = { enabled = true },
                    functionLikeReturnTypes = { enabled = true },
                    enumMemberValues = { enabled = true },
                },
            },
        },
    },

    biome = {
        cmd = { 'bun', 'x', 'biome', 'lsp-proxy' },
        root_dir = function(bufnr, on_dir)
            local root = find_js_project_root(bufnr)
            if root then
                if file_exists(root.root_dir .. '/biome.json') or file_exists(root.root_dir .. '/biome.jsonc') then
                    on_dir(root.root_dir)
                end
            end
        end,
    },

    texlab = {
        settings = {
            -- https://github.com/latex-lsp/texlab/wiki/Configuration
            texlab = {
                completion = { matcher = "prefix-ignore-case" },
            },
        },
    },
}
