function setup_go
    if [ -d "$GOPATH" ]
        fish_add_path "$GOPATH/bin"
    end
end
set -ax setup_list go
