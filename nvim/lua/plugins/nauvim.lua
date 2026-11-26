return {
    {
        "nauvim/nauvim-plugin",
        lazy = false,
        cond = os.getenv("NAUVIM_CONNECTION") ~= nil,
        config = function()
            vim.g.clipboard = require("nauvim-plugin.clipboard").provider
        end,
    },
}
