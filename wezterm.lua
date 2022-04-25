local wezterm = require 'wezterm';
return {
    font = wezterm.font("Source Han Code JP L"),
    font_size = 15,
    colors = {
        foreground = "#eeeeee",
        background = "#282828",
        selection_bg = "#293b44",
        selection_fg = "#ffffff",

        ansi = {"#282828", "#f43753", "#c9d05c", "#ffc24b", "#b3deef", "#d3b987", "#73cef4", "#eeeeee"},
        brights = {"#1d1d1d", "#f43753", "#c9d05c", "#ffc24b", "#b3deef", "#d3b987", "#73cef4", "#ffffff"},
    },

    scrollback_lines = 4096,

    keys = {
        {key="h", mods="CTRL", action={SendKey={key="Backspace"}}},

        {key="x", mods="CTRL|SHIFT", action="DisableDefaultAssignment"},
        {key="Insert", mods="CTRL", action="DisableDefaultAssignment"},
        {key="Insert", mods="SHIFT", action="DisableDefaultAssignment"},
        {key="Insert", mods="CTRL|SHIFT", action="ActivateCopyMode"},

        {key="-", mods="SUPER", action="DisableDefaultAssignment"},
        {key="-", mods="CTRL", action="DisableDefaultAssignment"},
        {key="=", mods="SUPER", action="DisableDefaultAssignment"},
        {key="=", mods="CTRL", action="DisableDefaultAssignment"},
    },

    audible_bell = "Disabled",

    exit_behavior = "Close",

    tab_bar_at_bottom = true,
}
