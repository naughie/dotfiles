set FNM_PATH $FNM_DIR/bin
if [ -d "$FNM_PATH" ]
    set PATH "$FNM_PATH" $PATH
    fnm env --shell fish | source
end
