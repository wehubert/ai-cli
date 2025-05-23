<h4 align="center">
  <img src="assets/logo.png" width=300/><br>
</h4>
<h4 align="center">Customizable OpenAI command line client written in Rust</h4>

## Table of contents
* [Installation](#installation)
* [How to use](#how-to-use)
* [Troubleshooting](#troubleshooting)
* [Contributing](#contributing)
* [License](#license)

## Installation

`ai-cli` can only be built and installed from sources. To install the application correctly:

* Instal latest Rust:

  The latest Rust toolchain is required to build and install `ai-cli`. Get Rust using official instruction: [Install rust](https://www.rust-lang.org/tools/install).

* Clone ai-cli repository:

  ```console
  git clone git@github.com:wehubert/ai-cli.git
  ```

* Build and install using cargo:

  ```console
  cargo install --path ai-cli
  ```

## How to use

Run `ai-cli` for the first time to generate default configuration and view usage details.

```console
ai-cli --help
```

```console
Usage: ai-cli <COMMAND>

Commands:
  chat   Ask a generic question
  howto  Get bash command that solves the problem
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The default configuration contains two assistants: chat (general chat assistant), howto (bash command generator). Learn how to change the configuration and define your own assistants by following the [Assistants configuration](#assistants-configuration).

### Set OpenAI API key

Before the usage you need to obtain and set your [OpenAI API key](https://help.openai.com/en/articles/4936850-where-do-i-find-my-openai-api-key). There are two ways to set your API key. Exporting the `OPENAI_API_KEY` system variable or updating `config.toml` configuration file. The first option is recommended for security reasons.

**Note** that using the OpenAI API is paid on a per-request basis and depends on the number of tokens and model used. Check current [pricing information](https://openai.com/api/pricing/). The default assistants are configured using the currently most cost-effective model, which is `gpt-4.1-nano`.

* Set system variable (recommended):

  Export `OPENAI_API_KEY` system variable according to your operating system and shell environment:
  ```console
  export OPENAI_API_KEY=<put-your-key-here>
  ```

* Update `config.toml` file:

  In some cases it might be more convenient to put your OpenAI API key directly to `config.toml` file. It is located in the default user configuration directory:

  ```console
  ~/.config/ai-cli/config.toml
  ```

  Edit the configuration file by adding `openai_api_key` to it:

  ```console
  openai_api_key = <put-your-key-here>
  ```

### Assistants configuration

Assistants configuration is stored in the `config.toml` located in the default user configuration directory:

```console
~/.config/ai-cli/config.toml
```

assistant configuration requires 6 mandatory fields and looks like this:

```toml
[[assistants]]
name = "chat"
help = "Ask a generic question"
model = "gpt-4.1-nano"
max_tokens = 100
system_message = "You are a helpful assistant"
prefix = ""
```

* name - assistant name that will be used to generate `ai-cli` subcommand
* help - assistant description that is used in `ai-cli --help`,
* model - OpenAI [model](https://platform.openai.com/docs/models) to use,
* max_tokens - the maximum number of [tokens](https://platform.openai.com/tokenizer) that can be generated in the chat completion,
* system_message - allows to guide the model on desired responses.
* prefix - prepend to the message sent to the OpenAI.


### Usage examples
 
* `howto` assistant

  Configuration:
  
  ```toml
  [[assistants]]
  name = "howto"
  help = "Get bash command that solves the problem"
  model = "gpt-4.1-nano"
  max_tokens = 100
  system_message = "Answer only with linux bash command no markdown"
  prefix = "How to"
  ```

  Example:

  ```console
  ai-cli howto install rust toolchain
  ```

  ```console
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

* `chat` assistant

  Configuration:

  ```toml
  [[assistants]]
  name = "chat"
  help = "Ask a generic question"
  model = "gpt-4.1-nano"
  max_tokens = 100
  system_message = "You are a helpful assistant"
  prefix = ""
  ```

  Example:

  ```console
  ai-cli chat write a short, one-sentence description of the CLI tool for communicating with AI
  ```

  ```console
  A CLI tool for communicating with AI enables users to effortlessly interact with artificial intelligence through command-line commands for seamless, efficient conversations
  ```

* `code` assistant

  Configuration:

  ```toml
  [[assistants]]
  name = "code"
  help = "Write a code snippet that meets the requirements"
  model = "gpt-4o"
  max_tokens = 200
  system_message = "Only respond with a code snippet that meets the requirements"
  prefix = ""
  ```

  Example:
  ```console
  ai-cli code simple echo server in python
  ```

  ```python
  import socket

  def start_echo_server(host='localhost', port=12345):
      with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
          server_socket.bind((host, port))
          server_socket.listen()
          print(f"Echo server listening on {host}:{port}")
          
          while True:
              client_socket, addr = server_socket.accept()
              with client_socket:
                  print(f"Connected by {addr}")
                  while True:
                      data = client_socket.recv(1024)
                      if not data:
                          break
                      client_socket.sendall(data)

  if __name__ == "__main__":
      start_echo_server()
  ```

## Troubleshooting

* `Error: ApiError(ApiError{message: "You didn't provide an API key"})` - Did you forget to export OpenAI API key? No worries, learn how to set an API key by following the [Set OpenAI API key](#set-openai-api-key) section.

* ``Error: BadTomlData(TomlError{message: "missing field `help`"})`` - `BadTomlData` indicates that there is an syntax error in your `config.toml` file or some mandatory field is missing. Fix your configuration file by following the [Assistants configuration](#assistants-configuration) section.

* ``Error: ApiError(ApiError { message: "The model `gpt-1` does not exist or you do not have access to it."`` - You chose not existing chat model. Fix `config.toml` by providing correct chat model. Please check available models and their [pricing information](https://platform.openai.com/docs/pricing).

To enable verbose debug logs set `debug_mode` to `true` in `config.toml` file:
```toml
debug_mode = true
```

## Contributing

Any contribution is welcome. Submit a [Pull Request](https://github.com/wehubert/ai-cli/pulls) or report an [Issue](https://github.com/wehubert/ai-cli/issues).

## License

`ai-cli` is released under the MIT License. See [LICENSE](LICENSE) for details.