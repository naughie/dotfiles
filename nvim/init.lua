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

vim.opt.laststatus = 2
vim.opt.statusline = '%#StatusLineFilename#%{"@"}%f%m%#warningmsg#%=%#StatusLineCursorPosition#(%l,%c)/(%L,%{strwidth(getline("."))})'
vim.cmd.highlight({ 'StatusLineFilename', 'ctermfg=46', 'guifg=#00ff00' })
vim.cmd.highlight({ 'StatusLineCursorPosition', 'ctermfg=184', 'guifg=#d7d700' })

vim.opt.swapfile = false
vim.opt.backupskip = { '/tmp/*', '/private/tmp/*' }
vim.opt.backup = false
vim.opt.writebackup = false

vim.opt.visualbell = true

vim.opt.updatetime = 300

vim.opt.timeoutlen = 500

vim.opt.shortmess:append('c')

vim.opt.guicursor = { a = 'hor5-blinkon500-blinkoff500-blinkwait0' }

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
vim.keymap.set('n', '<Space>q', ':qa<CR>', silent_opt)

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
