local api = vim.api

local helper = {}

function helper.show_in_float(cmd, cwd)
    local buf = api.nvim_create_buf(false, true)
    api.nvim_set_option_value("bufhidden", "wipe", { buf = buf })
    api.nvim_set_option_value("filetype", "log", { buf = buf })
    api.nvim_set_option_value("modifiable", false, { buf = buf })

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

    api.nvim_buf_set_keymap(buf, "n", "q", ":q<CR>", { silent = true })

    vim.fn.jobstart(cmd, {
        term = true,
        cwd = cwd,
        height = height,
        width = width,
    })

    vim.api.nvim_feedkeys("G", "n", true)
end

function helper.run_cargo_cmd(subcmd, opts)
    local cmd, cwd

    if #opts.fargs == 0 then
        cmd = { "cargo", subcmd }

        local buf_path = vim.api.nvim_buf_get_name(0)
        if buf_path == "" then
            cwd = vim.fn.getcwd()
        else
            cwd = vim.fs.dirname(buf_path)
        end
    else
        cmd = { "cargo", subcmd, "-p", opts.fargs[1] }
        cwd = vim.fn.getcwd()
    end
    table.insert(cmd, "--all-features")

    helper.show_in_float(cmd, cwd)
end

local cargo = {
    test = function(opts) helper.run_cargo_cmd("test", opts) end,
    check = function(opts) helper.run_cargo_cmd("clippy", opts) end,
}

api.nvim_create_user_command("Clippy", cargo.check, { nargs = "?", force = true })
api.nvim_create_user_command("Rustest", cargo.test, { nargs = "?", force = true })
