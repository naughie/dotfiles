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
            return current_dir
        end

        if file_exists(current_dir .. '/package.json') then
            return nil
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
    root_dir = function(bufnr, on_dir)
        local root = find_js_project_root(bufnr)
        if root then on_dir(root) end
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
}
