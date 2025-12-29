# Use the official Rust image (Debian-based)
FROM rust:latest

# Install system dependencies required by c2rust
# c2rust requires clang and llvm-dev
RUN apt-get update && apt-get install -y \
    llvm \
    clang \
    libclang-dev \
    cmake \
    python3 \
    python3-pip \
    libssl-dev \
    pkg-config

# Install compiledb to help generate compile_commands.json inside the container
RUN pip3 install compiledb --break-system-packages

# Install c2rust
# We link the LLVM configuration so c2rust can find the system libraries
ENV LLVM_CONFIG_PATH=/usr/bin/llvm-config
RUN cargo install c2rust

# Set the working directory
WORKDIR /app

# Default command
CMD ["/bin/bash"]