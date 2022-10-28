> ### 🚧 Caution:
>
> ⚠️ The project is a **Work in Progress** and doesn't work.

<h1 align="center">Regg Template Engine</h1>

<p align="center"> A simple template engine written in Rust 🦀 </p>

Regg's goal is to output JavaScript code that can be used with [Vite's plugin API](https://vitejs.dev/guide/api-plugin.html#transforming-custom-file-types) to generate static HTML. <br />

## Table of Content

- [Development](#development)
- [Roadmap](#roadmap)
  - [Templating Engine](#templating-engine)
  - [Miscellaneous](#miscellaneous)
- [Syntax Guide](#syntax-guide)
  - [Frontmatter](#frontmatter)
  - [Expressions](#expressions)
  - [Markup Expressions](#markup-expressions)
- [Context Free Grammer](#context-free-grammar)
- [Inspirations](#inspirations)

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

### Templating Engine

- [ ] Scanner (or Lexer or Tokenizer)
  - [x] Support Markup Tags
  - [x] Support Markup inside expressions  `` (` `` and `` `) ``
  - [x] Support Expressions 
  - [x] Support Code Blocks — code between `---` and `---` at the start
  - [ ] Support escaped expression syntax — `\{` and `\}`
- [ ] Parser
- [ ] Traverser
- [ ] Transformer
- [ ] Code Generator
- [ ] Compiler

### Miscellaneous
- [x] Devise a Context Free Grammer
  - [ ] Port to Backus-Naur Form [Optional]
- [x] CLI with Clap-rs
  - [x] REPL Mode
  - [x] Take File Path to run files
  - [ ] Output Files [nice to have]
- [ ] Precomiled Node.js addone
  - [ ] NAPI-rs integration

## Syntax Guide
### Frontmatter
```astro
---
  const greeting = 'hello world!';
  const navItems = ['home', 'about', 'contact']
---
```
The code inside the Frontmatter Fence Tokens (`---`) is called a Codeblock. <br />
This is different from an expression that goes inside `{` and `}` as it does not need to be checked if it returns a string or number.

### Expressions
```astro
<main>
  <h1>{greeting}</h1>
</main>
```
Stuff between `{` and `}` is a JavaScript expressions <br />
The expressions should get evaluated into a string or number or markup.

### Markup Expressions
```astro
<main>
  {
    navItems.map(navItem => {
      return (`<nav>{navItem}</nav>`)
    })
  }
</main> 
```
A JavaScript Expression can return a Markup Expression. <br />
A Markup Expression tells Regg that this markup contains JavaScript expressions that need to be evaluated. <br />
The syntax is adding the Markup Expression between `` `( `` and `` `) ``, this is valid JavaScript unlike JSX. <br />
Although the expression part is Regg specific syntax that is evaluated by the templating engine at build time.

## Context Free Grammar
```
Frontmatter -> CodeBlock
HTMLElement -> OpeningTagStart TextNode* OpeningTagEnd (HTMLElement* | TextNode) (ClosingTag | SelfClosingTag)
TextNode    -> Expression* (HTMLExprStart HTMLElement* HTMLExprEnd)* Expression*
```

```
OpeningTagStart      -> <foo             ; foo = \[A-Za-z]\
OpeningTagEnd        -> >                ;
SelfClosingTag       -> />               ;
ClosingTag           -> </foo>           ; foo = \[A-Za-z]\

CodeBlock            -> --- bar ---      ; bar = \*\
Expression           -> { bar }          ; bar = \*\
HTMLExprStart        -> (`               ;
HTMLExprEnd          -> `)               ;
```

## Inspirations

- [Crafting Interpreters](https://github.com/munificent/craftinginterpreters)
- [Astro](https://github.com/withastro/compiler)
- [Abell](https://github.com/abelljs/abell/)
