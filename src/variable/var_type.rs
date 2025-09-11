use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the type of a variable in the plugin system.
///
/// VariableType defines all supported data types that can be passed between
/// plugins and the host application. It provides a type-safe way to represent
/// different kinds of data.
///
/// # Variants
///
/// * `Let` - Unspecified type (default)
/// * `Int` - Integer types (signed/unsigned with various sizes)
/// * `Float` - Floating point types (f32, f64)
/// * `Bool` - Boolean values
/// * `Char` - Unicode characters
/// * `String` - UTF-8 strings
/// * `List` - Lists/arrays of variables
///
/// # Examples
///
/// ```rust
/// use plux_rs::variable::VariableType;
///
/// // Create specific types
/// let int_type = VariableType::I32;
/// let string_type = VariableType::String;
/// let list_type = VariableType::List;
///
/// // Use in function arguments
/// let arg = plux_rs::function::Arg::new("count", VariableType::I32);
/// ```
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableType {
    /// Unspecified or default type
    #[default]
    Let,
    /// Integer types (signed/unsigned)
    Int(VariableIntType),
    /// Floating point types
    Float(VariableFloatType),
    /// Boolean values
    Bool,
    /// Unicode characters
    Char,
    /// UTF-8 strings
    String,
    /// Lists/arrays of variables
    List,
}

/// Represents integer types with size and signedness information.
///
/// VariableIntType distinguishes between signed and unsigned integer types
/// of various sizes supported by the system.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableIntType {
    /// Signed integer types
    Signed(VariableSignedIntType),
    /// Unsigned integer types
    Unsigned(VariableUnsignedIntType),
}

/// Represents signed integer types of various sizes.
///
/// This enum defines all supported signed integer types in the system.
/// I32 is the default type.
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableSignedIntType {
    /// 8-bit signed integer
    I8,
    /// 16-bit signed integer
    I16,
    /// 32-bit signed integer (default)
    #[default]
    I32,
    /// 64-bit signed integer
    I64,
}

/// Represents unsigned integer types of various sizes.
///
/// This enum defines all supported unsigned integer types in the system.
/// U32 is the default type.
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableUnsignedIntType {
    /// 8-bit unsigned integer
    U8,
    /// 16-bit unsigned integer
    U16,
    /// 32-bit unsigned integer (default)
    #[default]
    U32,
    /// 64-bit unsigned integer
    U64,
}

/// Represents floating point types of various sizes.
///
/// This enum defines all supported floating point types in the system.
/// F32 is the default type.
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub enum VariableFloatType {
    /// 32-bit floating point (default)
    #[default]
    F32,
    /// 64-bit floating point
    F64,
}

impl VariableType {
    /// 8-bit signed integer type
    pub const I8: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I8));
    /// 16-bit signed integer type
    pub const I16: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I16));
    /// 32-bit signed integer type
    pub const I32: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I32));
    /// 64-bit signed integer type
    pub const I64: VariableType =
        VariableType::Int(VariableIntType::Signed(VariableSignedIntType::I64));
    /// 8-bit unsigned integer type
    pub const U8: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U8));
    /// 16-bit unsigned integer type
    pub const U16: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U16));
    /// 32-bit unsigned integer type
    pub const U32: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U32));
    /// 64-bit unsigned integer type
    pub const U64: VariableType =
        VariableType::Int(VariableIntType::Unsigned(VariableUnsignedIntType::U64));
    /// 32-bit floating point type
    pub const F32: VariableType = VariableType::Float(VariableFloatType::F32);
    /// 64-bit floating point type
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
