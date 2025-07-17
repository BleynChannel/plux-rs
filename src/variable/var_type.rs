use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableType {
    #[default]
    Let,
    Int(VariableIntType),
    Float(VariableFloatType),
    Bool,
    Char,
    String,
    List,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableIntType {
    Signed(VariableSignedIntType),
    Unsigned(VariableUnsignedIntType),
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableSignedIntType {
    I8,
    I16,
    #[default]
    I32,
    I64,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableUnsignedIntType {
    U8,
    U16,
    #[default]
    U32,
    U64,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableFloatType {
    #[default]
    F32,
    F64,
}

impl VariableType {
    pub const I8: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I8));
    pub const I16: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I16));
    pub const I32: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I32));
    pub const I64: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I64));
    pub const U8: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U8));
    pub const U16: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U16));
    pub const U32: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U32));
    pub const U64: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U16));
    pub const F32: VariableType = VariableType::Float(VariableFloatType::F32);
    pub const F64: VariableType = VariableType::Float(VariableFloatType::F64);
}

impl Default for VariableIntType {
    fn default() -> Self {
        Self::Signed(Default::default())
    }
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(t) => write!(f, "{t}"),
            Self::Float(t) => write!(f, "{t}"),
            ty => write!(f, "{ty:?}"),
        }
    }
}

impl Display for VariableIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Signed(t) => write!(f, "{t}"),
            Self::Unsigned(t) => write!(f, "{t}"),
        }
    }
}

impl Display for VariableFloatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for VariableSignedIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for VariableUnsignedIntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
