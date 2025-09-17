return {
    {
        "naughie/nvim-router.nvim",
        lazy = true,
        opts = function(plugin)
            return {
                plugin_dir = plugin.dir,
                ns = { "buffer-switcher", "lazy-filer", "rg-fancy" },
                force = false,
            }
        end,
    },
}
