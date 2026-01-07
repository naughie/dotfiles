local filetypes = {
    {
        "bash",
        { "sh", "bash" },
    },
    {
        "bibtex",
        { "bib" },
    },
    {
        "caddy",
        { "caddy" },
    },
    {
        "cpp",
        { "c", "cpp" },
    },
    {
        "css",
        { "css" },
    },
    {
        "csv",
        { "csv", "tsv" },
    },
    {
      "dart",
      { "dart" },
    },
    {
        "dockerfile",
        { "dockerfile" },
    },
    {
        "fish",
        { "fish" },
    },
    {
        "go",
        { "go" },
    },
    {
        "html",
        { "html" },
    },
    {
        "javascript",
        { "javascript", "javascriptreact" },
    },
    {
        "json",
        { "json", "jsonc" },
    },
    {
        "latex",
        { "latex", "plaintex", "tex" },
    },
    {
        "lua",
        { "lua" },
    },
    {
        "markdown",
        { "markdown" },
    },
    {
        "python",
        { "python" },
    },
    {
        "rust",
        { "rust" },
    },
    {
        "scss",
        { "scss" },
    },
    {
        "ssh_config",
        { "sshconfig" },
    },
    {
        "toml",
        { "toml" },
    },
    {
        "tsx",
        { "typescriptreact" },
    },
    {
        "typescript",
        { "typescript" },
    },
    {
        "vim",
        { "vim" },
    },
    {
        "vimdoc",
        { "help" },
    },
    {
        "xml",
        { "xml" },
    },
    {
        "yaml",
        { "yaml" },
    },
}

local treesitter_langs = {}
local ft_patterns = {}

for _, ft_table in ipairs(filetypes) do
    table.insert(treesitter_langs, ft_table[1])

    for _, ft in ipairs(ft_table[2]) do
        table.insert(ft_patterns, ft)
    end
end

if vim.fn.executable("tree-sitter") == 0 then
    return {}
end


return {
    {
        "nvim-treesitter/nvim-treesitter",
        build = ":TSUpdate",
        lazy = false,
        branch = "main",
        config = function () 
            local configs = require("nvim-treesitter")

            configs.install(treesitter_langs)

            local treesitter_au = vim.api.nvim_create_augroup("nvim-treesitter-setup", { clear = true })
            vim.api.nvim_create_autocmd({ "FileType" }, {
                pattern = ft_patterns,
                group = treesitter_au,
                callback = function()
                    vim.treesitter.start()
                end,
            })
        end
    },
}
