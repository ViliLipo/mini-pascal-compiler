pub enum OpKind {
    Addition,
    NumArithmetic(NumArithmetic),
    Modulo,
    Relational(Relational),
    BoolArithmetic(BoolArithmetic),
}

pub enum NumArithmetic {
    Minus,
    Division,
    Multi,
}

pub enum Relational {
    Equal,
    NotEqual,
    Smaller,
    SmallerE,
    Larger,
    LargerE,
}

pub enum BoolArithmetic {
    Or,
    And,
}

pub fn string_as_opkind(op: &String) -> Option<OpKind> {
    match op.as_str() {
        "+" => Some(OpKind::Addition),
        "*" => Some(OpKind::NumArithmetic(NumArithmetic::Multi)),
        "/" => Some(OpKind::NumArithmetic(NumArithmetic::Division)),
        "-" => Some(OpKind::NumArithmetic(NumArithmetic::Minus)),
        "%" => Some(OpKind::Modulo),
        "=" => Some(OpKind::Relational(Relational::Equal)),
        "<>" => Some(OpKind::Relational(Relational::NotEqual)),
        "<" => Some(OpKind::Relational(Relational::Smaller)),
        "<=" => Some(OpKind::Relational(Relational::SmallerE)),
        ">=" => Some(OpKind::Relational(Relational::LargerE)),
        ">" => Some(OpKind::Relational(Relational::Larger)),
        "or" => Some(OpKind::BoolArithmetic(BoolArithmetic::Or)),
        "and" => Some(OpKind::BoolArithmetic(BoolArithmetic::And)),
        _ => None,
    }
}
