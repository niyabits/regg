use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    StartExprToken,        // `{{`
    EndExprToken,          // `}}`
    FrontmatterFenceToken, // opening or closing `---`

    /* Literals */
    JsExpr,   // JavaScript Expression between `{{ ` and ` }}`
    JsBlock,  // JavaScript Code Block between `---` and `---`
    CssBlock, // CSS Code Block between `<style>` and `</style>`
    Markup,   // HTML Code in between

    /* Keyword */
    StyleTagOpen,  // <style>
    StyleTagClose, // </style>

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
