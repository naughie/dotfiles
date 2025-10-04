if [ -d "$RBENV_ROOT" ]
    fish_add_path "$RBENV_ROOT/bin"
    eval "$(rbenv init -)"
end
