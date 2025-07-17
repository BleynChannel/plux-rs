use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::utils::ParseVariableError;

#[derive(Default, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
pub enum Variable {
    #[default]
    Null,
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    String(String),
    List(Vec<Variable>),
}

pub trait FromVariable {
    type Output;
    type RefOutput<'a>
    where
        Self: 'a;
    type MutOutput<'a>
    where
        Self: 'a;

    fn from_var(var: Variable) -> Result<Self::Output, ParseVariableError>;
    fn from_var_ref(var: &Variable) -> Result<Self::RefOutput<'_>, ParseVariableError>;
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
    pub fn is_null(&self) -> bool {
        match self {
            Variable::Null => true,
            _ => false,
        }
    }
}

impl Variable {
    pub fn parse<F>(self) -> F::Output
    where
        F: FromVariable,
    {
        F::from_var(self).unwrap()
    }

    pub fn parse_ref<F>(&self) -> F::RefOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_ref(self).unwrap()
    }

    pub fn parse_mut<F>(&mut self) -> F::MutOutput<'_>
    where
        F: FromVariable,
    {
        F::from_var_mut(self).unwrap()
    }

    pub fn try_parse<F>(self) -> Result<F::Output, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var(self)
    }

    pub fn try_parse_ref<F>(&self) -> Result<F::RefOutput<'_>, ParseVariableError>
    where
        F: FromVariable,
    {
        F::from_var_ref(self)
    }

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
    type RefOutput<'a> = Vec<T::RefOutput<'a>> where T: 'a;
    type MutOutput<'a> = Vec<T::MutOutput<'a>> where T: 'a;

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
