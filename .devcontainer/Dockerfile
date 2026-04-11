FROM mcr.microsoft.com/devcontainers/rust:trixie
RUN echo "deb http://apt.llvm.org/trixie/ llvm-toolchain-trixie-21 main" >> /etc/apt/sources.list.d/llvm.list
RUN echo "deb-src http://apt.llvm.org/trixie/ llvm-toolchain-trixie-21 main" >> /etc/apt/sources.list.d/llvm.list
RUN wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key > /etc/apt/trusted.gpg.d/apt.llvm.org.asc
RUN apt-get update && apt-get install -y llvm-21-dev libpolly-21-dev
ENV LLVM_SYS_211_PREFIX=/usr/lib/llvm-21
