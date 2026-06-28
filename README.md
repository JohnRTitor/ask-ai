# ask-ai

A fast, backend-agnostic terminal helper for interacting with local and remote Large Language Models (LLMs). It provides a familiar `ollama run` / `llama-cli` style interface for **any OpenAI-compatible API**, while offering advanced features like real-time streaming, terminal markdown rendering, syntax highlighting, and an interactive REPL with session persistence.

## Features

- **Backend-Agnostic**: Works with Ollama, `llama.cpp`, vLLM, LM Studio, LiteLLM, or any OpenAI-compatible server.
- **Rich Terminal UI**: Streams responses directly to your terminal with real-time markdown rendering and syntax highlighting for code blocks.
- **Interactive REPL**: Maintain multi-turn conversations with history, session saving, and meta-commands (`/help`, `/system`, `/clear`).
- **Comprehensive Sampling Control**: Fine-tune your generation with granular CLI flags that pass straight through to your backend (including `llama.cpp` specific flags).
- **Thinking Token Support**: Automatically detects `<think>...</think>` tags (from models like DeepSeek or Qwen) and renders them unobtrusively.
- **Metrics**: Displays token usage, generation latency, and tokens-per-second (TPS) after generation.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then clone this repository and build:

```bash
git clone https://github.com/yourusername/ask-ai.git
cd ask-ai
cargo build --release
```

The optimized binary will be located at `target/release/ask-ai`.

## Usage

### Commands

*   `ask-ai run <MODEL> [PROMPT]` - Generate a single response. If no prompt is provided, it reads from `stdin`.
*   `ask-ai chat [MODEL]` - Start an interactive chat session (REPL).
*   `ask-ai models` - List available models on the backend.
*   `ask-ai ps` - List currently loaded/running models (Ollama only).
*   `ask-ai pull <MODEL>` - Download a model (Ollama only).
*   `ask-ai stop [MODEL]` - Unload a model from memory (Ollama only).
*   `ask-ai config <COMMAND>` - Manage the local configuration file.
*   `ask-ai backend <COMMAND>` - Switch between backends.

### Examples

**Single-shot generation:**
```bash
ask-ai run qwen3:8b "Explain transformers in simple terms."
```

**Piping data from standard input:**
```bash
cat main.rs | ask-ai run qwen3:8b "Refactor this code to be more idiomatic."
```

**Starting a chat session:**
```bash
ask-ai chat qwen3:8b
```

## Configuration

`ask-ai` uses a TOML configuration file located at `~/.config/ask-ai/config.toml`. All CLI arguments override the defaults set in this file.

Example `config.toml`:

```toml
backend = "ollama"
base_url = "http://localhost:11434/v1"
api_key = "none"
model = "qwen3:8b"

temperature = 0.7
top_p = 0.9
max_tokens = 4096
```

## Sampling Parameters

You can adjust the behavior of the model using CLI flags. These are sent directly to the backend.

| Flag / Parameter | Description |
| :--- | :--- |
| `--temperature <FLOAT>` | Controls randomness. Higher values increase diversity, lower values make output more deterministic. |
| `--top-p <FLOAT>` | Nucleus sampling. Filters out tokens that do not contribute to the top P probability mass. |
| `--top-k <INT>` | Limits the next token selection to the K most probable tokens. |
| `--min-p <FLOAT>` | Sets a minimum probability threshold relative to the most likely token (supported by `llama.cpp`/Ollama). |
| `--repeat-penalty <FLOAT>`| Penalizes the model for repeating recently generated tokens. |
| `--presence-penalty <FLOAT>`| Penalizes tokens based on whether they appear in the text so far. |
| `--frequency-penalty <FLOAT>`| Penalizes tokens based on their frequency in the text so far. |
| `--mirostat <INT>` | Enables Mirostat sampling (0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0). |
| `--mirostat-tau <FLOAT>`| Controls the target entropy for Mirostat. |
| `--mirostat-eta <FLOAT>`| Controls the learning rate for Mirostat. |
| `--seed <INT>` | Sets a random seed for deterministic generation. |
| `--max-tokens <INT>` | Limits the maximum number of tokens the model can generate. |
| `--context <INT>` | Sets the size of the context window (Ollama specific: `num_ctx`). |
| `--stream` / `--no-stream` | Forces the backend to stream or buffer the response. |

### Global Flags

*   `--backend <NAME>`: Overrides the configured backend (e.g., `openai`, `ollama`, `llamacpp`).
*   `--base-url <URL>`: Overrides the backend's API base URL.
*   `--api-key <KEY>`: Overrides the API key used for authentication.

## Backends

The tool supports three primary backend profiles to handle API quirks:
1.  **`openai`** (Default): Standard OpenAI `/v1/chat/completions` API. Works with almost all proxies (vLLM, LM Studio, etc.).
2.  **`ollama`**: Natively handles Ollama's structure. Exposes `pull`, `ps`, and `stop` commands via `/api/*`, while using `/v1/` for generation. Nests parameters inside an `options` object automatically.
3.  **`llamacpp`**: Passes advanced sampling parameters directly in the root JSON body expected by the `llama.cpp` server.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
