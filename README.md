# lazycli
ChatGPT in your terminal...cuz why not?

# Usage
`lazycli` utilizes the GPT-3 model to generate completions. In order to use it, you need an [OpenAI API](https://openai.com/blog/openai-api) Key

On Linux:

```bash
export OPENAI_API_KEY='sk...'
```

Build the binary by running `cargo run --path .`

Run lazycli --help to get a full list of options:

```sh
$ lazycli --help
"ChatGPT in the terminal, 'nuff said"

Usage: lazycli [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...  The prompt given by the user

Options:
  -e, --exec    Execute generated text, such as shell commands
  -h, --help     Print help
  -V, --version  Print version
```

