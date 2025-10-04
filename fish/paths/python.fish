function setup_python
    test -x "$CONDA_HOME/bin/conda" && eval "$("$CONDA_HOME/bin/conda" shell.fish hook)"
end
set -ax setup_list python
