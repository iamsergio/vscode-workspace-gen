# SPDX-License-Identifier: MIT

cargo build && \
cargo build -F qt && \
echo "You can now run ./target/debug/vscode-workspace-gen"
