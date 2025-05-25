# Azure OpenAI CLI Tool

This project is a command-line interface (CLI) tool written in Rust that interacts with the Azure OpenAI API. It takes a string input from the user and generates a response using the capabilities of Azure OpenAI.

## Project Structure

```
azure-openai-cli
├── src
│   └── main.rs       # Entry point of the application
├── Cargo.toml        # Rust project configuration
└── README.md         # Project documentation
```

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- An Azure account with access to the Azure OpenAI service.

### Setup

1. Clone the repository:

   ```
   git clone <repository-url>
   cd azure-openai-cli
   ```

2. Configure your Azure OpenAI credentials. You will need to set the following environment variables:

   ```
   export AZURE_OPENAI_API_KEY=<your-api-key>
   export AZURE_OPENAI_ENDPOINT=<your-endpoint-url>
   ```

3. Build the project:

   ```
   cargo build
   ```

### Running the CLI Tool

To run the CLI tool, use the following command:

```
cargo run -- <your-input-string>
```

Replace `<your-input-string>` with the string you want to send to the Azure OpenAI API.

### Example

```
cargo run -- "Hello, how are you?"
```

This will send the input string to the Azure OpenAI API and print the response to the console.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.