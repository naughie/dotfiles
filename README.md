# Examples

Install:

```
/home/me $ git clone https://github.com/naughie/dotfiles.git
/home/me $ cd dotfiles
/home/me/dotfiles $ ./tools/make-$(uname -m)-$(uname -s)
/home/me/dotfiles $ ls "$HOME/.config/terminfo/" | xargs -I{} tic "$HOME/.config/terminfo/{}"

# e.g.
/home/me/dotfiles $ ./tools/make-x86_64-Linux
```

## Prerequisites

```
$ apt install -y git curl build-essential musl-dev
```
