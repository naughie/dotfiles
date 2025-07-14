local api = vim.api

function show_in_float(cmd, cwd)
    local buf = api.nvim_create_buf(false, true)
    api.nvim_buf_set_option(buf, "bufhidden", "wipe")
    api.nvim_buf_set_option(buf, "filetype", "log")

    local width = math.floor(api.nvim_get_option("columns") * 0.5)
    local height = math.floor(api.nvim_get_option("lines") * 0.8)
    local row = math.floor((api.nvim_get_option("lines") - height) / 2)
    local col = math.floor((api.nvim_get_option("columns") - width) / 2)

    local title = table.concat(cmd, " ")

    local win = api.nvim_open_win(buf, true, {
        relative = "editor",
        width = width,
        height = height,
        row = row,
        col = col,
        style = "minimal",
        border = "rounded",
        title = title,
        title_pos = "center",
    })

    api.nvim_buf_set_keymap(buf, "n", "q", "<Cmd>close<CR>", { silent = true })

    vim.fn.jobstart(cmd, {
        term = true,
        cwd = cwd,
        height = height,
        width = width,
    })
end

function cargocheck(opts)
    local cmd, cwd

    if #opts.fargs == 0 then
        cmd = { 'cargo', 'clippy' }

        local buf_path = vim.api.nvim_buf_get_name(0)
        if buf_path == "" then
            cwd = vim.fn.getcwd()
        else
            cwd = vim.fs.dirname(buf_path)
        end
    else
        cmd = { 'cargo', 'clippy', '-p', opts.fargs[1] }
        cwd = vim.fn.getcwd()
    end

    show_in_float(cmd, cwd)
end

api.nvim_create_user_command('Clippy', cargocheck, { nargs = '?', force = true })
