#[derive(Clone, Copy, Debug)]
pub enum Value {
    Undef,
    Number(f64),
}

pub type ConstantPool = Vec<Value>;
