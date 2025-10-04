# init anaconda (python)
set -x CONDA_HOME $HOME/etc/anaconda3
test -x $CONDA_HOME/bin/conda && eval "$($CONDA_HOME/bin/conda shell.fish hook)"
# fish_add_path $CONDA_HOME/bin
