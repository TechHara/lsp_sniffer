# LSP Sniffer

This tool allows one to intercept communications from a language server protocol ([LSP](https://langserver.org/)) client and server.

## How it works
This simply creates a thin wrapper that relays all the stdio communications between the client and the server, while logging them to files.

## Compile
```shell
cargo build -r
```

## Usage
1. locate LSP server binary.
2. name the server binary as `lsp_server` while replacing the original filename with this project's binary file.

### VSCode Example
Let's say we want to sniff LSP communications between [VSCode](https://code.visualstudio.com/) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.
1. change directory to the folder that contains `rust-analyzer` binary file for the VSCode extension following [this](https://rust-analyzer.github.io/manual.html#vs-code), e.g.,
```shell
cd ~/.vscode/extensions/rust-lang.rust-analyzer-0.3.1877-darwin-arm64/server
```
2. rename the server binary
```shell
mv rust-analyzer lsp_server
```
3. copy this project's binary as the original server binary name
```shell
cp /path/to/this/project/target/release/lsp_sniffer rust-analyzer
```
4. restart VSCode. If everything is successful, you will see `msg_from_client.txt` and `msg_from_server.txt` files in the directory where the sniffer binary sits.

## Limitations
Currently, this works only if the LSP server binary is a natively compiled executable, such as `rust-analyzer`. However, some language servers are distributed as non-native executable, such as `jar`. This project does not support such cases out of the box, but theoretically it should be simple to support such cases as well. In addition, it only supports communications via stdio, but again, it should be trivial to extend and support other channels as well, such as TCP or WebSocket.