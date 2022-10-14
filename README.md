> ### 🚧 Caution:
>
> ⚠️ The project is a **Work in Progress** and doesn't work.

<h1 align="center">Regg Template Engine</h1>

<p align="center"> A simple template engine written in Rust 🦀 </p>

Regg's goal is to output optimized JavaScript code that can be used with [Vite's plugin API](https://vitejs.dev/guide/api-plugin.html#transforming-custom-file-types) to generate static HTML. <br />

## Development

#### Run a `.regg` file:

```sh
cargo run -- <FILEPATH>
```

#### REPL:

```sh
cargo run
```

## Roadmap

### Regg Core Functionality

Please see `example/main.regg` <br />
That's all I am planning with the proof of concept, in the future I might gradually expand to more things.

1. Scanner (or Lexer or Tokenizer)
2. Syntax Tree
3. Parser

## Inspirations

Regg takes a lot of inspiration from [Abell](https://github.com/abelljs/abell/), the project would simply, not have been possible without Abell <3
