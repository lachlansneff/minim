
#[derive(Clone)]
pub enum MemAccess {
    Index(u32),
    Cell(Box<MemAccess>),
}

#[derive(Clone)]
pub enum Literal {
    True,
    False,
    Index,
    LastCell,
}

#[derive(Clone)]
pub enum Value {
    Mem(MemAccess),
    Constant(i64),
    Literal(Literal),
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Multiply(Box<Value>, Box<Value>),
    Divide(Box<Value>, Box<Value>),
}

#[derive(Clone)]
pub enum ValueType {
    Unsigned,
    Signed,
    Ascii,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Label {
    Number(u32),
    Ascii(char),
}

#[derive(Clone)]
pub enum Equality {
    LessThan,
    GreaterThan,
    EqualTo,
}

#[derive(Clone)]
pub enum Instruction {
    AssignCell(MemAccess, Value),
    Output(ValueType, Value),
    Input(ValueType, Value),
    CreateLabel(Label),
    GotoLabel(Label),
    TernaryAssign(MemAccess, Value, Equality, Value, Value, Value),
    TernaryGoto(Label, Value, Equality, Value, Value, Value),
}
