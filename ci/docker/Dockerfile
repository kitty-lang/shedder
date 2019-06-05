FROM archlinux/base:latest
MAINTAINER Matthieu Le Brazidec <matthieu@lebrazidec.email>

# LLVM (7.0)
ENV LLVM_SYS_70_PREFIX=/usr/lib/llvm-7
RUN pacman -Sy --noconfirm llvm7

# Rust
RUN pacman -Sy --noconfirm rustup
RUN rustup default stable
