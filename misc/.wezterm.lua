local wezterm = require 'wezterm';

local config = {
    font = wezterm.font_with_fallback {
        { family = "SauceCodePro Nerd Font", weight = "Light", assume_emoji_presentation = false },
        { family = "Source Han Code JP L", scale = 0.975, assume_emoji_presentation = false },
        { family = "Source Han Code JP L", scale = 0.975, assume_emoji_presentation = true },
    },
    font_size = 15,

    -- color_scheme = 'everforest light',
    color_scheme = 'everforest dark',

    window_background_opacity = 0.96,
    text_background_opacity = 0.4,

    use_fancy_tab_bar = false,

    scrollback_lines = 4096,

    keys = {
        {key="h", mods="CTRL", action={SendKey={key="Backspace"}}},

        {key="t", mods="CTRL|SHIFT", action="DisableDefaultAssignment"},
        {key="Tab", mods="CTRL", action="DisableDefaultAssignment"},
        {key="Tab", mods="CTRL|SHIFT", action="DisableDefaultAssignment"},
        {key="w", mods="CTRL|SHIFT", action="DisableDefaultAssignment"},

        {key="x", mods="CTRL|SHIFT", action="DisableDefaultAssignment"},
        {key="Insert", mods="CTRL", action="DisableDefaultAssignment"},
        {key="Insert", mods="SHIFT", action="DisableDefaultAssignment"},
        {key="Insert", mods="CTRL|SHIFT", action="ActivateCopyMode"},

        {key="-", mods="SUPER", action="DisableDefaultAssignment"},
        {key="-", mods="CTRL", action="DisableDefaultAssignment"},
        {key="=", mods="SUPER", action="DisableDefaultAssignment"},
        {key="=", mods="CTRL", action="DisableDefaultAssignment"},

        {key="raw:36", mods="CTRL", action={SendKey={key="raw:36", mods="CTRL"}}},
        {key="raw:31", mods="CTRL", action={SendKey={key="raw:31", mods="CTRL"}}},
    },

    audible_bell = "Disabled",

    exit_behavior = "Close",

    tab_bar_at_bottom = true,

    warn_about_missing_glyphs = false,

    show_update_window = false,

    window_decorations = "NONE",
}

config.color_schemes = {
    ['tender'] = {
        foreground = "#eeeeee",
        background = "#282828",
        selection_fg = "#ffffff",
        selection_bg = "#293b44",

        ansi = {"#282828", "#f43753", "#c9d05c", "#ffc24b", "#b3deef", "#d3b987", "#73cef4", "#eeeeee"},
        brights = {"#1d1d1d", "#f43753", "#c9d05c", "#ffc24b", "#b3deef", "#d3b987", "#73cef4", "#ffffff"},
    },

    ['everforest dark'] = {
        foreground = "#d3c6aa",
        background = "#333c43",
        selection_bg = "rgba(92, 63, 79, 50%)", -- #5c3f4f

        ansi = {"#333c43", "#e67e80", "#a7c080", "#dbbc7f", "#7fbbb3", "#d699b6", "#83c092", "#d3b987"},
        brights = {"#333c43", "#e67e80", "#a7c080", "#dbbc7f", "#7fbbb3", "#d699b6", "#83c092", "#d3b987"},

        tab_bar = {
            background = '#3a464c',

            active_tab = {
                bg_color = '#5c3f4f',
                fg_color = '#d3c6aa',
            },
            inactive_tab = {
                bg_color = '#48584e',
                fg_color = '#d3c6aa',
            },
            inactive_tab_hover = {
                bg_color = '#555f66',
                fg_color = '#d3c6aa',
                italic = false,
            },
            new_tab = {
                bg_color = '#5d6b66',
                fg_color = '#d3c6aa',
            },
            new_tab_hover = {
                bg_color = '#555f66',
                fg_color = '#d3c6aa',
                italic = false,
            },
        },
    },

    ['everforest light'] = {
        foreground = "#5c6a72",
        background = "#f3ead3",
        selection_bg = "#e1e4bd",

        ansi = {"#f3ead3", "#f85552", "#8da101", "#dfa000", "#3a94c5", "#df69ba", "#35a77c", "#5c6a72"},
        brights = {"#f3ead3", "#f85552", "#8da101", "#dfa000", "#3a94c5", "#df69ba", "#35a77c", "#5c6a72"},

        tab_bar = {
            background = '#ddd8be',

            active_tab = {
                bg_color = '#e1e4bd',
                fg_color = '#5c6a72',
            },
            inactive_tab = {
                bg_color = '#f4dbd0',
                fg_color = '#5c6a72',
            },
            inactive_tab_hover = {
                bg_color = '#b9c0ab',
                fg_color = '#5c6a72',
                italic = false,
            },
            new_tab = {
                bg_color = '#eae4ca',
                fg_color = '#5c6a72',
            },
            new_tab_hover = {
                bg_color = '#b9c0ab',
                fg_color = '#5c6a72',
                italic = false,
            },
        },
    },
}

return config
