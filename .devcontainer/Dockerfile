FROM nvcr.io/nvidia/tensorrt-llm/release:1.1.0rc3

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set up Codex config
COPY codex/ /root/.codex/
