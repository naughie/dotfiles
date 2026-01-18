FROM debian:trixie-slim

RUN apt update && apt install -y git curl build-essential musl-dev

RUN groupadd --gid 1234 testgroup && \
    useradd --home-dir /home/testuser --gid testgroup --shell /bin/sh --create-home --uid 1234 testuser

USER testuser

WORKDIR /home/testuser

RUN mkdir etc && \
    cd etc && \
    git clone https://github.com/naughie/dotfiles.git && \
    cd dotfiles && \
    curl -L 'https://github.com/naughie/dotfiles/releases/latest/download/make-x86_64-unknown-linux-gnu.tar.xz' | tar xvJ --strip-components=1 && \
    ./make link -e ./shell/env && \
    ./make profile -e ./shell/env && \
    ./make install -e ./shell/env && \
    ls "$HOME/.config/terminfo/" | xargs -I{} tic "$HOME/.config/terminfo/{}"

ENTRYPOINT ["/bin/sh", "-il"]
