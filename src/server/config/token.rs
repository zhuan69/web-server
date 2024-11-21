enum LiteralKind {
    Number(usize),
    Str(String),
}

enum TokenKind {
    Literal(LiteralKind),
    OpeningCurlyBracket(char),
    CloseCurlyBraces(char),
    OpenBracket(char),
    CloseBracket(char),
    Colon(char),
    Semicolon(char),
    Symbol(char),
    EOF,
}
