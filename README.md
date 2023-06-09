# lazycli
AI-assistant in your terminal

# Installation

Install this project by running the following command:
```bash
curl -fsSL https://raw.githubusercontent.com/rijulrr/lazycli/main/install.sh | sh -
```

# Usage
`lazycli` utilizes the GPT-3 model to generate completions. In order to use it, you need an [OpenAI API](https://openai.com/blog/openai-api) Key

On Linux/macOS:

```bash
export OPENAI_API_KEY='sk-...'
```

You can also add the key to your shells configuration file (ending in `rc`)

Run `lazycli --help` to get a full list of options:

```sh
$ lazycli --help
"Why do all the work when AI can instead?"

Usage: lazycli [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...  The prompt given by the user

Options:
  -e, --exec    Execute generated text, such as shell commands
  -h, --help     Print help
  -V, --version  Print version
```

