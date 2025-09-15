FROM mcr.microsoft.com/devcontainers/rust:trixie

RUN apt-get update && apt-get install -y wget gnupg2 lsb-release 

RUN wget -O - https://apt.llvm.org/llvm.sh | bash -s -- 18 all
ENV PATH="$PATH:/lib/llvm-18/bin"
