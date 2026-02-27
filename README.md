# proto-claude-code-plugin

A [proto](https://moonrepo.dev/proto) WASM plugin for managing
[Claude Code](https://github.com/anthropics/claude-code) installations.

## Installation

Add to `.prototools`:

```toml
[plugins]
claude-code = "github://BarretoTech/proto-claude-code-plugin"
```

Or with a pinned plugin version:

```toml
[plugins]
claude-code = "github://BarretoTech/proto-claude-code-plugin@v0.1.0"
```

## Usage

Install the latest version:

```bash
proto install claude-code
```

Install a specific version:

```bash
proto install claude-code 2.1.62
```

List available versions:

```bash
proto versions claude-code
```

Pin a version:

```bash
proto pin claude-code 2.1.62
```

After installation, the `claude` command is available:

```bash
claude --version
claude --help
```

## Configuration

Custom distribution URL can be set in `.prototools`:

```toml
[tools.claude-code]
dist-url = "https://your-mirror.example.com/claude-code-releases"
```

## Supported Platforms

| Platform        | Architecture |
|-----------------|-------------|
| macOS           | arm64, x64  |
| Linux (glibc)   | arm64, x64  |
| Linux (musl)    | arm64, x64  |
| Windows         | x64, arm64  |

## Development

Setup toolchain:

```bash
proto install rust
rustup target add wasm32-wasip1
```

Build the plugin:

```bash
cargo build --target wasm32-wasip1
```

Test locally with proto:

```toml
# .prototools
[plugins]
claude-code = "file://./target/wasm32-wasip1/debug/claude_code_plugin.wasm"
```

```bash
proto --log trace install claude-code
```

## License

MIT
