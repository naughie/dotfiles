# Examples

Install:

```bash
cd $HOME/etc
git clone https://github.com/naughie/dotfiles.git
cd dotfiles

curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-x86_64-unknown-linux-gnu.tar.xz' | tar xvJ --strip-components=1
./make install -e ./shell/env
./make link -e ./shell/env
./make profile -e ./shell/env
ls "$HOME/.config/terminfo/" | xargs -I{} tic "$HOME/.config/terminfo/{}"
```

Here, select the installation URL depending on your host environment:

```bash
# x86_64 Linux, Gnu
curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-x86_64-unknown-linux-gnu.tar.xz' | tar xvJ --strip-components=1
# x86_64 Linux, Musl
curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-x86_64-unknown-linux-musl.tar.xz' | tar xvJ --strip-components=1
# aarch64 Linux, Gnu
curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-aarch64-unknown-linux-gnu.tar.xz' | tar xvJ --strip-components=1
# Apple Silicon
curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-aarch64-apple-darwin.tar.xz' | tar xvJ --strip-components=1
# Intel Mac
curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-x86_64-apple-darwin.tar.xz' | tar xvJ --strip-components=1
```

## Prerequisites

```
$ apt install -y git curl build-essential musl-dev
$ brew install fish neovim
```

- [Golang](https://go.dev/doc/install) for macOS

# Other setups

- [`xremap-config`](https://github.com/naughie/xremap-config)
