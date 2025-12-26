FROM mcr.microsoft.com/devcontainers/rust:trixie

RUN echo "deb http://apt.llvm.org/trixie/ llvm-toolchain-trixie-21 main" >> /etc/apt/sources.list.d/llvm.list
RUN echo "deb-src http://apt.llvm.org/trixie/ llvm-toolchain-trixie-21 main" >> /etc/apt/sources.list.d/llvm.list
RUN wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | sudo tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc
RUN apt-get update && apt-get install -y libllvm21 llvm-21 clang-21 clang-tools-21 libpolly-21-dev lld-21
ENV PATH=$PATH:/lib/llvm-21/bin
ENV LLVM_SYS_211_PREFIX=/lib/llvm-21
