use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::utils::ParseVariableError;

/// Represents a dynamically typed value that can be passed between plugins.
///
/// Variable is the core data type used for communication between plugins and the host.
/// It supports all common data types and provides type-safe conversion methods.
///
/// # Variants
///
/// * `Null` - Represents a null/empty value (default)
/// * `I8`, `I16`, `I32`, `I64` - Signed integer types
/// * `U8`, `U16`, `U32`, `U64` - Unsigned integer types
/// * `F32`, `F64` - Floating point types
/// * `Bool` - Boolean values
/// * `Char` - Unicode characters
/// * `String` - UTF-8 strings
/// * `List` - Lists/arrays of variables
///
/// # Examples
///
/// ```rust
/// use plux_rs::variable::Variable;
///
/// // Create variables of different types
/// let num = Variable::I32(42);
/// let text = Variable::String("Hello".to_string());
/// let flag = Variable::Bool(true);
/// let list = Variable::List(vec![num, text, flag]);
///
/// // Convert from Rust types
/// let var1: Variable = 42_i32.into();
/// let var2: Variable = "hello".into();
/// let var3: Variable = vec![1, 2, 3].into();
/// ```
#[derive(Default, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub enum Variable {
    /// Null/empty value
    #[default]
    Null,
    /// 8-bit signed integer
    I8(i8),
    /// 16-bit signed integer
    I16(i16),
    /// 32-bit signed integer
    I32(i32),
    /// 64-bit signed integer
    I64(i64),
    /// 8-bit unsigned integer
    U8(u8),
    /// 16-bit unsigned integer
    U16(u16),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit unsigned integer
    U64(u64),
    /// 32-bit floating point
    F32(f32),
    /// 64-bit floating point
    F64(f64),
    /// Boolean value
    Bool(bool),
    /// Unicode character
    Char(char),
    /// UTF-8 string
    String(String),
    /// List of variables
    List(Vec<Variable>),
}

/// Trait for converting Variables to specific Rust types.
///
/// FromVariable provides methods to safely convert Variables to their corresponding
/// Rust types. It supports owned, borrowed, and mutable borrowed conversions.
///
/// # Type Parameters
///
/// * `Output` - The owned type to convert to
/// * `RefOutput<'a>` - The borrowed type to convert to
/// * `MutOutput<'a>` - The mutable borrowed type to convert to
///
/// # Required Methods
///
/// * `from_var` - Convert an owned Variable to the target type
/// * `from_var_ref` - Convert a borrowed Variable to the target type
/// * `from_var_mut` - Convert a mutable borrowed Variable to the target type
///
/// # Example
///
/// ```rust
/// use plux_rs::variable::{Variable, FromVariable};
///
/// let var = Variable::I32(42);
///
/// // Convert to owned value
/// let owned: i32 = i32::from_var(var).unwrap();
///
/// // Convert to borrowed value
/// let var_ref = &Variable::I32(42);
/// let borrowed: &i32 = i32::from_var_ref(var_ref).unwrap();
/// ```
pub trait FromVariable {
    /// The owned output type
    type Output;
    /// The borrowed output type
    type RefOutput<'a>
    where
        Self: 'a;
    /// The mutable borrowed output type
    type MutOutput<'a>
    where
        Self: 'a;

    /// Convert an owned Variable to the target type.
    ///
    /// # Parameters
    ///
    /// * `var` - The Variable to convert
    ///
    /// # Returns
    ///
    /// Returns `Result<Self::Output, ParseVariableError>` containing the converted value
    /// or an error if the conversion fails.
    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError>;

    /// Convert a borrowed Variable to the target type.
    ///
    /// # Parameters
    ///
    /// * `var` - The Variable to convert
    ///
    /// # Returns
    ///
    /// Returns `Result<Self::RefOutput<'_>, ParseVariableError>` containing the converted value
    /// or an error if the conversion fails.
    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError>;

    /// Convert a mutable borrowed Variable to the target type.
    ///
    /// # Parameters
    ///
    /// * `var` - The Variable to convert
    ///
    /// # Returns
    ///
    /// Returns `Result<Self::MutOutput<'_>, ParseVariableError>` containing the converted value
    /// or an error if the conversion fails.
    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError>;
}

macro_rules! impl_from {
    ($ty:ty, $from:ident) => {
        impl From<$ty> for Variable {
            fn from(x: $ty) -> Self {
                Self::$from(x)
            }
        }
    };
}

macro_rules! impl_from_variable {
    ($ty:ty, $from:ident) => {
        impl FromVariable for $ty {
            type Output = Self;
            type RefOutput<'a> = &'a Self;
            type MutOutput<'a> = &'a mut Self;

            fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }

            fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }

            fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
                match var {
                    Variable::$from(x) => Ok(x),
                    _ => Err(ParseVariableError::new(stringify!($from))),
                }
            }
        }
    };
}

impl From<&str> for Variable {
    fn from(x: &str) -> Self {
        Self::String(x.to_string())
    }
}

impl<T> From<&[T]> for Variable
where
    T: Into<Variable> + Clone,
{
    fn from(x: &[T]) -> Self {
        Self::List(x.iter().cloned().map(|item| item.into()).collect())
    }
}

impl<T> From<Vec<T>> for Variable
where
    T: Into<Variable>,
{
    fn from(x: Vec<T>) -> Self {
        Self::List(x.into_iter().map(|item| item.into()).collect())
    }
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(f32, F32);
impl_from!(f64, F64);
impl_from!(bool, Bool);
impl_from!(char, Char);
impl_from!(String, String);

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Null => write!(f, "Null"),
            Variable::I8(v) => write!(f, "{v}"),
            Variable::I16(v) => write!(f, "{v}"),
            Variable::I32(v) => write!(f, "{v}"),
            Variable::I64(v) => write!(f, "{v}"),
            Variable::U8(v) => write!(f, "{v}"),
            Variable::U16(v) => write!(f, "{v}"),
            Variable::U32(v) => write!(f, "{v}"),
            Variable::U64(v) => write!(f, "{v}"),
            Variable::F32(v) => write!(f, "{v}"),
            Variable::F64(v) => write!(f, "{v}"),
            Variable::Bool(v) => write!(f, "{v}"),
            Variable::Char(v) => write!(f, "{v}"),
            Variable::String(v) => write!(f, "{v}"),
            Variable::List(v) => write!(f, "{v:?}"),
        }
    }
}

impl Variable {
    /// Check if the Variable represents a null value.
    ///
    /// # Returns
    ///
    /// Returns `true` if the Variable is `Variable::Null`, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::variable::Variable;
    ///
    /// let null_var = Variable::Null;
    /// let int_var = Variable::I32(42);
    ///
    /// assert!(null_var.is_null());
    /// assert!(!int_var.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            Variable::Null => true,
            _ => false,
        }
    }
}

impl Variable {
    /// Parse the Variable into a specific type (panics on error).
    ///
    /// This method converts the Variable to the specified type, panicking if
    /// the conversion fails. Use this when you're certain about the type.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns the converted value of type `F::Output`.
    ///
    /// # Panics
    ///
    /// Panics if the Variable cannot be converted to the target type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::variable::Variable;
    ///
    /// let var = Variable::I32(42);
    /// let num: i32 = var.parse::<i32>();
    /// assert_eq!(num, 42);
    /// ```
    pub fn parse<F>(self) -> F::Output
    where
        F: FromVariable,
    {
        F::from_var(self).unwrap()
    }

    /// Parse the Variable into a specific type by reference (panics on error).
    ///
    /// This method converts the Variable to the specified type without consuming it,
    /// panicking if the conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns a reference to the converted value of type `F::RefOutput<'_>`.
    ///
    /// # Panics
    ///
    /// Panics if the Variable cannot be converted to the target type.
    pub fn parse_ref<F>(&self) -> F::RefOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_ref(self).unwrap()
    }

    /// Parse the Variable into a specific type by mutable reference (panics on error).
    ///
    /// This method converts the Variable to the specified type without consuming it,
    /// panicking if the conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the converted value of type `F::MutOutput<'_>`.
    ///
    /// # Panics
    ///
    /// Panics if the Variable cannot be converted to the target type.
    pub fn parse_mut<F>(&mut self) -> F::MutOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_mut(self).unwrap()
    }

    /// Try to parse the Variable into a specific type.
    ///
    /// This method attempts to convert the Variable to the specified type,
    /// returning an error if the conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns `Result<F::Output, ParseVariableError>` containing the converted value
    /// or an error if the conversion fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use plux_rs::variable::Variable;
    ///
    /// let var = Variable::I32(42);
    /// match var.try_parse::<i32>() {
    ///     Ok(num) => println!("Parsed: {}", num),
    ///     Err(e) => println!("Parse error: {}", e),
    /// }
    /// ```
    pub fn try_parse<F>(self) -> Result<F::Output, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var(self)
    }

    /// Try to parse the Variable into a specific type by reference.
    ///
    /// This method attempts to convert the Variable to the specified type without consuming it,
    /// returning an error if the conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns `Result<F::RefOutput<'_>, ParseVariableError>` containing a reference to the
    /// converted value or an error if the conversion fails.
    pub fn try_parse_ref<F>(&self) -> Result<F::RefOutput<'_>, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var_ref(self)
    }

    /// Try to parse the Variable into a specific type by mutable reference.
    ///
    /// This method attempts to convert the Variable to the specified type without consuming it,
    /// returning an error if the conversion fails.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The target type that implements FromVariable
    ///
    /// # Returns
    ///
    /// Returns `Result<F::MutOutput<'_>, ParseVariableError>` containing a mutable reference to the
    /// converted value or an error if the conversion fails.
    pub fn try_parse_mut<F>(&mut self) -> Result<F::MutOutput<'_>, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var_mut(self)
    }
}

impl FromVariable for Vec<Variable> {
    type Output = Self;
    type RefOutput<'a> = &'a Self;
    type MutOutput<'a> = &'a mut Self;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }

    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }

    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => Ok(x),
            _ => Err(ParseVariableError::new("Vec<Variable>")),
        }
    }
}

impl<T> FromVariable for Vec<T>
where
    T: FromVariable,
{
    type Output = Vec<T::Output>;
    type RefOutput<'a>
        = Vec<T::RefOutput<'a>>
    where
        T: 'a;
    type MutOutput<'a>
        = Vec<T::MutOutput<'a>>
    where
        T: 'a;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.into_iter() {
                    arr.push(var.try_parse::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }

    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.iter() {
                    arr.push(var.try_parse_ref::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }

    fn from_var_mut(var: &mut Variable) -> Result<Self::MutOutput<'_>, ParseVariableError> {
        match var {
            Variable::List(x) => {
                let mut arr = vec![];
                for var in x.iter_mut() {
                    arr.push(var.try_parse_mut::<T>()?);
                }
                Ok(arr)
            }
            _ => Err(ParseVariableError::new("Vec<T>")),
        }
    }
}

impl_from_variable!(i8, I8);
impl_from_variable!(i16, I16);
impl_from_variable!(i32, I32);
impl_from_variable!(i64, I64);
impl_from_variable!(u8, U8);
impl_from_variable!(u16, U16);
impl_from_variable!(u32, U32);
impl_from_variable!(u64, U64);
impl_from_variable!(f32, F32);
impl_from_variable!(f64, F64);
impl_from_variable!(bool, Bool);
impl_from_variable!(char, Char);
impl_from_variable!(String, String);

#[test]
fn into() {
    let a = 10_i16;

    let b: Variable = a.into();
    assert_eq!(b, Variable::I16(10));
}

#[test]
fn parse() {
    let mut a: Variable = 10_i16.into();

    assert_eq!(a.clone().parse::<i16>(), 10);
    assert_eq!(a.parse_ref::<i16>(), &10);
    assert_eq!(a.parse_mut::<i16>(), &mut 10);

    match a.clone().try_parse::<i16>() {
        Ok(b) => assert_eq!(b, 10),
        Err(e) => panic!("{}", e),
    };

    match a.try_parse_ref::<i16>() {
        Ok(b) => assert_eq!(b, &10),
        Err(e) => panic!("{}", e),
    };

    match a.try_parse_mut::<i16>() {
        Ok(b) => assert_eq!(b, &mut 10),
        Err(e) => panic!("{}", e),
    };
}

#[test]
fn parse_vec() {
    let mut a: Variable = vec![10_i16].into();

    let b = a.parse_mut::<Vec<i16>>();

    assert_eq!(b, vec![&mut 10]);
}
