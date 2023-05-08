FROM debian:buster-slim

# Download LLVM

WORKDIR /root

RUN apt-get update &&\
    apt-get install -y --no-install-recommends cmake python3 curl xz-utils g++ git ninja-build ca-certificates &&\
    rm -rf /var/lib/apt/lists/*

RUN curl https://github.com/llvm/llvm-project/releases/download/llvmorg-13.0.1/llvm-project-13.0.1.src.tar.xz -L -o llvm.tar.xz &&\
    tar -xf llvm.tar.xz &&\
    rm llvm.tar.xz

# Build LLVM

WORKDIR /root/llvm-build

RUN cmake -S ../llvm-project-13.0.1.src/llvm -B . -G Ninja -DCMAKE_BUILD_TYPE=Release &&\
    ninja -C . install

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/root/llvm-buil/bin:$PATH \
    RUST_VERSION=1.69.0

# Install rust

WORKDIR /root

RUN apt-get update && apt-get install -y --no-install-recommends wget clang

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='bb31eaf643926b2ee9f4d8d6fc0e2835e03c0a60f34d324048aa194f0b29a71c' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='6626b90205d7fe7058754c8e993b7efd91dedc6833a11a225b296b7c2941194f' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='4ccaa7de6b8be1569f6b764acc28e84f5eca342f5162cd5c810891bff7ed7f74' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='34392b53a25c56435b411d3e575b63aab962034dd1409ba405e708610c829607' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.25.2/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

WORKDIR /root/llvm

# CMD ["cargo", "run"]
