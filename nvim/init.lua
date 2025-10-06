if vim.env.FNM_DIR then
    local fnm_node_host_prog = vim.env.FNM_DIR .. "/aliases/default/bin/neovim-node-host"
    if vim.fn.executable(fnm_node_host_prog) == 1 then
        vim.g.node_host_prog = fnm_node_host_prog
    end
end

vim.g.loaded_perl_provider = 0
vim.g.loaded_python3_provider = 0
vim.g.loaded_ruby_provider = 0

-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({
      { "Failed to clone lazy.nvim:\n", "ErrorMsg" },
      { out, "WarningMsg" },
      { "\nPress any key to exit..." },
    }, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)

-- Make sure to setup `mapleader` and `maplocalleader` before
-- loading lazy.nvim so that mappings are correct.
vim.g.mapleader = "\\"
vim.g.maplocalleader = "\\"

-- Setup lazy.nvim
require("lazy").setup({
  spec = {
    { import = "plugins" },
  },
  rocks = { enabled = false },
  -- colorscheme that will be used when installing plugins.
  install = { colorscheme = { "habamax" } },
  -- automatically check for plugin updates
  checker = { enabled = false },
})

require("lsp-setup").setup()

vim.opt.fencs = { 'utf-8', 'euc-jp', 'shift-jis', 'default' }

vim.opt.mouse = ''

vim.opt.termguicolors = true

vim.opt.number = true
vim.opt.ruler = true

vim.opt.hidden = true

vim.opt.spelllang= { 'en_us', 'cjk' }

vim.opt.expandtab = true
vim.opt.tabstop = 4
vim.opt.shiftwidth = 0
vim.opt.autoindent = true
vim.opt.smartindent = true

vim.opt.pumheight = 10

vim.opt.showmatch = true
vim.opt.matchtime = 1

vim.opt.hlsearch = false

vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.fileignorecase = true

vim.opt.scrolloff = 25

vim.opt.display = { 'lastline', 'uhex' }
vim.opt.wrap = true

vim.opt.conceallevel = 0

vim.opt.backspace = { 'indent', 'eol', 'start' }

local fillchar = "*"
local function gen_statusline()
    local function define_hl(base_hl_name, link_to)
        local tmp_hl = vim.api.nvim_get_hl(0, { name = link_to, link = false })
        local fg = string.format("#%06x", tmp_hl.fg or 0)

        local hl = "NaughieStatusLine" .. base_hl_name
        local hl_tick = "NaughieStatusLine" .. base_hl_name .. "Tick"

        vim.api.nvim_set_hl(0, hl, { reverse = true, fg = fg })
        vim.api.nvim_set_hl(0, hl_tick, { fg = fg })

        return function(content)
            local left_tick = "%#" .. hl_tick .. "#\u{e0b6}"
            local body = "%#" .. hl .. "#" .. content
            local right_tick = "%#" .. hl_tick .. "#\u{e0b4}"

            return left_tick .. body .. right_tick .. "%#StatusLine#"
        end
    end

    local fname = define_hl("Fname", "Operator")(" \u{eda5} %{v:lua.naughie_gen_path()}%m%r ")
    local mode = define_hl("Mode", "Keyword")(" %{v:lua.naughie_gen_mode()} ")
    local cursor = define_hl("Cursor", "Type")(" \u{ed00}(%l,%v)/(%L,%{strwidth(getline('.'))}) ")

    local home_dir = vim.env.HOME
    function naughie_gen_path()
        local path = vim.fn.expand("%")
        if path == "" then return "[NO\u{a0}NAME]" end

        local cwd = vim.uv.cwd()
        if string.find(path, cwd, 1, true) == 1 then
            path = "." .. string.sub(path, #cwd + 1)
        end

        if string.find(path, home_dir, 1, true) == 1 then
            path = "~" .. string.sub(path, #home_dir + 1)
        end

        local shorten = vim.fn.pathshorten(path)
        return shorten
    end

    local mode_icon = "\u{f4e1}\u{a0}"
    local mode_map = {
          ["n"]   = "NORMAL",
          ["i"]   = "INSERT",
          ["v"]   = "VISUAL",
          ["V"]   = "VISUAL\u{a0}L",
          ["\22"] = "VISUAL\u{a0}B", -- CTRL-V
          ["c"]   = "COMMAND",
          ["s"]   = "SELECT",
          ["S"]   = "SELECT\u{a0}L",
          ["\19"] = "SELECT\u{a0}B", -- CTRL-S
          ["R"]   = "REPLACE",
          ["t"]   = "TERMINAL",
    }
    local max_mode_len = 0
    for _, val in pairs(mode_map) do
        max_mode_len = math.max(max_mode_len, vim.fn.strwidth(val))
    end
    max_mode_len = max_mode_len + vim.fn.strwidth(mode_icon)
    function naughie_gen_mode()
        local mode = vim.api.nvim_get_mode().mode
        local mode_str = mode_icon .. (mode_map[mode] or mode)

        local diff = max_mode_len - vim.fn.strwidth(mode_str)
        local pad_left = math.floor(diff / 2)
        local pad_right = diff - pad_left

        return string.rep("\u{a0}", pad_left) .. mode_str .. string.rep("\u{a0}", pad_right)
    end

    local statusline = { fname, mode, cursor }

    return fillchar .. table.concat(statusline, "%=") .. fillchar
end
vim.opt.fillchars:append(string.format("stl:%s,stlnc:%s", fillchar, fillchar))
vim.opt.statusline = gen_statusline()
vim.opt.showmode = false
vim.opt.laststatus = 2

vim.opt.swapfile = false
vim.opt.backupskip = { '/tmp/*', '/private/tmp/*' }
vim.opt.backup = false
vim.opt.writebackup = false

vim.opt.visualbell = true

vim.opt.updatetime = 300

vim.opt.timeoutlen = 500

vim.opt.shortmess:append('c')

vim.opt.guicursor = 'a:hor5-blinkon500-blinkoff500-blinkwait0'

vim.opt.signcolumn = 'yes'

vim.opt.cursorline = true

vim.opt.completeopt:append({ 'menuone', 'noselect', 'popup' })



local silent_opt = { silent = true }

vim.keymap.set('n', '<BS>', '<C-w>h', silent_opt)
vim.keymap.set('n', '<C-h>', '<C-w>h', silent_opt)
vim.keymap.set('n', '<C-l>', '<C-w>l', silent_opt)
vim.keymap.set('n', '<C-k>', '<C-w>k', silent_opt)
vim.keymap.set('n', '<C-j>', '<C-w>j', silent_opt)

vim.keymap.set({ 'n', 'v' }, 'j', 'gj', silent_opt)
vim.keymap.set({ 'n', 'v' }, 'k', 'gk', silent_opt)
vim.keymap.set({ 'n', 'v' }, 'gj', 'j', silent_opt)
vim.keymap.set({ 'n', 'v' }, 'gk', 'k', silent_opt)

vim.keymap.set({ 'n', 'v' }, '<Space>h', '^', silent_opt)
vim.keymap.set({ 'n', 'v' }, '<Space>l', '$', silent_opt)

vim.keymap.set('n', '<Space>w', ':w<CR>', silent_opt)
vim.keymap.set('n', '<Space>q', function()
    local current_sock = vim.v.servername
    local global_sock = vim.env.NVIM_SOCK
    if global_sock and global_sock == current_sock then
        vim.cmd('detach')
    else
        vim.cmd('qa')
    end
end, silent_opt)

vim.keymap.set('n', '<Bar>', function() vim.cmd('vsplit | vsplit') end, silent_opt)

vim.keymap.set('v', '<Space>y', '"+y', silent_opt)
vim.keymap.set('n', 'Y', 'y$', silent_opt)

vim.keymap.set('n', '<Space>v', '<C-v>', silent_opt)

vim.keymap.set('n', '<C-p>', '"zdd<Up>"zP', silent_opt)
vim.keymap.set('n', '<C-n>', '"zdd"zp', silent_opt)

vim.keymap.set('n', '<Space><Space>', [["zyiw:let @/ = '\<' . @z . '\>'<CR>:set hlsearch<CR>]], silent_opt)
vim.keymap.set('n', '<Space>n', ':noh<CR>', silent_opt)

vim.keymap.set('n', 'n', 'nzz', silent_opt)
vim.keymap.set('n', 'N', 'Nzz', silent_opt)
vim.keymap.set('c', '/', function()
    if vim.fn.getcmdtype() == '/' then return '\\/' end
    return '/'
end, { expr = true })
vim.keymap.set('c', '?', function()
    if vim.fn.getcmdtype() == '?' then return '\\?' end
    return '?'
end, { expr = true })

function naughie_open_file(file)
    local curr_buf = vim.api.nvim_buf_get_name(0)
    if curr_buf == '' then
        vim.cmd('e ' .. vim.fn.fnameescape(file))
        return
    end

    local wins = vim.api.nvim_tabpage_list_wins(0)
    if #wins == 0 or #wins == 1 then return end

    local fname_modified = vim.fn.fnamemodify(file, ':p')
    for _, win in ipairs(wins) do
        local buf = vim.api.nvim_win_get_buf(win)
        local bufname = vim.api.nvim_buf_get_name(buf)
        if bufname == fname_modified then return end
        if bufname == '' then
            vim.api.nvim_set_current_win(win)
            vim.cmd('e ' .. vim.fn.fnameescape(file))
            return
        end
    end
end

function naughie_open_dir(dir)
    local curr_cwd = vim.uv.cwd()
    if curr_cwd == dir then return end

    vim.cmd('tabnew | vsplit | vsplit | cd ' .. vim.fn.fnameescape(dir))
end

local eq_win_augroup = vim.api.nvim_create_augroup('NaughieEqualizeWindows', { clear = true }),
vim.api.nvim_create_autocmd('UIEnter', {
    group = eq_win_augroup,
    callback = function()
        vim.cmd('wincmd =')
    end,
})
