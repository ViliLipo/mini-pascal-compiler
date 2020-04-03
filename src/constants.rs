pub struct Constants {}

pub fn get_keywords<'a>() -> Vec<&'a str> {
    vec![
        "var",
        "and",
        "or",
        "not",
        "if",
        "then",
        "else",
        "of",
        "while",
        "do",
        "begin",
        "end",
        "array",
        "procedure",
        "function",
        "program",
        "assert",
        "return",
    ]
}

pub fn get_special_symbols<'a>() -> Vec<&'a str> {
    vec![
        "+", "-", "*", "/", "%", "=", "<>", "<", "<=", ">=", ">", "(", ")", "[", "]", ".", ",", ";",
    ]
}

pub fn get_predefined_identifiers<'a>() -> Vec<&'a str> {
    vec![
        "Boolean", "false", "true", "integer", "real", "string", "size", "read", "writeln",
    ]
}

