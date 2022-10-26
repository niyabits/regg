use std::fmt;

// Inspirations
// Astro Compiler:  https://github.com/withastro/compiler/blob/5d3923f3802b411eb9072f274ffcb9c04f9d4be1/internal/token.go#L24
// Lox:             https://github.com/munificent/craftinginterpreters/blob/01e6f5b8f3e5dfa65674c2f9cf4700d73ab41cf8/java/com/craftinginterpreters/lox/TokenType.java

#[derive(Debug)]
pub enum TokenType {
    // Any Text nodes which can contain a JSExpression
    TextToken,

    /* HTML */
    OpeningTagStart,   // `<div` | `<span`
    OpeningTagEnd,     // `>`
    ClosingTag,        // `</div>` | `</span>`
    SelfClosingTagEnd, //  />`

    /* Regg Specific */
    Expression,    // Code Expression: anything between `{` and `}`
    CodeBlock,     // Code Block: anything between two `---`
    HTMLExprStart, // (`
    HTMLExprEnd,   // `)

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/* Design Note: */
// Astro's Tokenizer breaks down some tokens as follows –
// Expression       -> StartExpressionToken  TextToken* EndExpressionToken
// CodeBlock        -> FrontmatterFenceToken TextToken* FrontmatterFenceToken
//
// The reason I don't do this is, I see code expressions and code blocks similar to string literals in Lox,
// in the Lox TokenType Bob doesn't tokenize the quotes,
// I don't see any benefits to tokenizing `{` and `---` seperately,
// if such requirement comes up in the future I am open to having those tokens seperate
// Until then I'd like to enjoy having less Tokens to deal with.

/* Regg's Context Free Grammar */
// Frontmatter -> CodeBlock
// HTMLElement ->  OpeningTagStart TextNode* OpeningTagEnd (HTMLElement* | TextNode) (ClosingTag | SelfClosingTag)
// TextNode -> Expression* TextToken* Expression*
//
// CodeBlock            -> --- bar ---      ; bar = \*\
// OpeningTagStart      -> <foo             ; foo = \[A-Za-z]\
// OpeningTagEnd        -> >
// ClosingTag           -> </foo>           ; foo = \[A-Za-z]\
// SelfClosingTag       -> />
// Expression           -> { bar }        ; bar = \*\
