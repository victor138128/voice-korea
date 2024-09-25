FROM ubuntu

WORKDIR /root

SHELL ["/bin/bash", "-c"]
RUN apt update
RUN ln -fs /usr/share/zoneinfo/Asia/Seoul /etc/localtime
RUN apt install curl clang perl build-essential libc6:amd64 libstdc++6:amd64 libbz2-1.0:amd64 libncurses6:amd64 libncurses-dev openjdk-17-jdk cmake ninja-build pkg-config libgtk-3-dev curl clang perl build-essential apt-utils wget git unzip xz-utils zip libglu1-mesa libssl-dev -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . "$HOME/.cargo/env" && cargo install --git https://github.com/hackartists/dioxus.git --branch feature/addr-env dioxus-cli
RUN . "$HOME/.cargo/env" && rustup target add wasm32-unknown-unknown

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

RUN . "$HOME/.nvm/nvm.sh" && nvm install --lts

RUN mv ~/.bashrc ~/.profile

CMD ["/usr/bin/bash"]
