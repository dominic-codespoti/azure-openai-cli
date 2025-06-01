# Azure OpenAI CLI Tool

A modular, extensible command-line interface (CLI) tool written in Rust for interacting with Azure OpenAI and other LLM providers. Supports streaming output, provider plugins, and easy configuration.

## Features
- Modular provider system (easily add new LLM providers)
- Streaming output to stdout
- Configurable via CLI or config file
- Async, fast, and ready for scripting

## Project Structure

```
azure-openai-cli
├── src
│   ├── main.rs       # Entry point of the application
│   ├── config.rs     # Configuration management
│   ├── provider.rs   # Provider trait and registry
│   └── provider/
│       └── azure.rs  # Azure OpenAI provider implementation
├── Cargo.toml        # Rust project configuration
└── README.md         # Project documentation
```

## Installation

### From crates.io (recommended)

```
cargo install azure-openai-cli
ln -s ~/.cargo/bin/azure-openai-cli ~/.cargo/bin/chat
```

### Local installation (development)

```
cargo install --path .
ln -s ~/.cargo/bin/azure-openai-cli ~/.cargo/bin/chat
```

Make sure `~/.cargo/bin` is in your `PATH` (add `export PATH="$HOME/.cargo/bin:$PATH"` to your `~/.zshrc` or `~/.bashrc` if needed).

## Configuration

You can configure the CLI using the `config` subcommand or by editing the config file directly.

### CLI Config Commands

- Show current config:
  ```sh
  azure-openai-cli config show
  # or
  chat config show
  ```

- Set a config value:
  ```sh
  azure-openai-cli config set <key> <value>
  # or
  chat config set azure_api_key sk-...  # example
  ```
  
  Supported keys: `provider`, `azure_endpoint`, `azure_api_key`, `azure_deployment`

## Usage

Send a prompt to the LLM (default provider: Azure):

```
chat "Hello, how are you?"
```

With options:

```
chat --max-tokens 256 --temperature 0.7 "Tell me a joke about Rust."
```

## Example

```
$ chat "What is the capital of France?"
Paris.
```

## Environment Variables

You can also set credentials via environment variables:

```
export AZURE_OPENAI_API_KEY=sk-...
export AZURE_OPENAI_ENDPOINT=https://.../openai/deployments/...
```

## Adding Providers

To add a new provider, implement the `LLMProvider` trait in a new file under `src/provider/`, and register it in `get_provider` in `provider.rs`.

## Publishing to crates.io

1. Update `Cargo.toml` with your repository, homepage, and author info.
2. Bump the version if needed.
3. Login to crates.io:
   ```sh
   cargo login
   ```
4. Publish:
   ```sh
   cargo publish
   ```

## License

This project is licensed under the MIT OR Apache-2.0 License. See the LICENSE file for more details.