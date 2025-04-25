//! Provides a way to embed and access const JSON in Rust code, using a single `macro_rules`
//! declaration, and no dependencies, so it is quick to compile. See [`const_json`].
#![no_std]
#![forbid(missing_docs, unsafe_code)]

use core::ops::Index;

/// The result of a [`const_json`] macro call.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Json<'a> {
    /// `null`
    Null(()),
    /// `true` or `false`
    Bool(bool),
    /// A floating point number
    Float(f64),
    /// A 64-bit signed integer
    Int(i64),
    /// A string
    Str(&'a str),
    /// An untyped Json array
    Array(&'a [Json<'a>]),
    /// A Json key-value map
    Object(&'a [(&'a str, Json<'a>)]),
}

impl Json<'_> {
    const fn string_eq(l: &str, r: &str) -> bool {
        if l.len() != r.len() {
            return false;
        }
        let mut idx = 0;
        while idx < l.len() {
            if l.as_bytes()[idx] != r.as_bytes()[idx] {
                return false;
            }
            idx += 1;
        }
        return true;
    }

    /// Gets a value stored at the given key.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self` is not a [`Json::Object`]
    /// - the key could not be found
    pub const fn get_val(&self, key: &str) -> &Self {
        match self {
            Self::Object(obj) => {
                let mut i = 0;
                while i < obj.len() {
                    let (k, v) = &obj[i];
                    if Self::string_eq(k, key) {
                        return v;
                    }
                    i += 1;
                }
                panic!("key not found");
            }
            _ => panic!("wrong variant"),
        }
    }

    /// Gets a value stored at the given index.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self` is not a [`Json::Array`]
    /// - the index was out of range
    pub const fn get_idx(&self, index: usize) -> &Self {
        match self {
            Self::Array(arr) => &arr[index],
            _ => panic!("wrong variant"),
        }
    }

    /// Unwraps a Null value.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a [`Json::Null`]
    pub const fn null(&self) -> () {
        match *self {
            Self::Null(inner) => inner,
            _ => panic!("wrong variant"),
        }
    }

    /// Unwraps a Bool value.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a [`Json::Bool`]
    pub const fn bool(&self) -> bool {
        match *self {
            Self::Bool(inner) => inner,
            _ => panic!("wrong variant"),
        }
    }

    /// Unwraps a Float value.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a [`Json::Float`]
    pub const fn float(&self) -> f64 {
        match *self {
            Self::Float(inner) => inner,
            Self::Int(inner) => inner as f64,
            _ => panic!("wrong variant"),
        }
    }

    /// Unwraps an Int value.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a [`Json::Int`]
    pub const fn int(&self) -> i64 {
        match *self {
            Self::Int(inner) => inner,
            _ => panic!("wrong variant"),
        }
    }

    /// Unwraps a Str value.
    ///
    /// # Panics
    ///
    /// Panics if `self` is not a [`Json::Str`]
    pub const fn str(&self) -> &str {
        match *self {
            Self::Str(inner) => inner,
            _ => panic!("wrong variant"),
        }
    }
}

/// Declares a borrowed JSON structure without allocation at compile time. Valid JSON syntax works,
/// as well as one-token Rust expressions (such as a variable name). If you want to embed a more
/// complex expression, just surround it with parentheses.
/// ```rust
/// use const_json::{Json, const_json};
///
/// const JSON: Json = const_json!({
///     "null": null,
///     "bool": true,
///     "float": 12.3,
///     "int": 42,
///     "str": "Hello, World!",
///     "array": [1, null],
///     "object": {
///         "inner_bool": false,
///         "inner_str": "foo bar"
///     },
///
///     "variable": VARIABLE,
///     // Has to be surrounded in parentheses if it is a complex expression
///     "function_result": (10 + 4)
/// });
///
/// const VARIABLE: i64 = 10;
/// ```
#[macro_export]
macro_rules! const_json {
    (null) => {$crate::Json::Null(())};

    ([$($json:tt),*]) => {$crate::Json::Array(&[$($crate::const_json!($json)),*])};
    ({$($key:literal: $val:tt),*}) => {
        $crate::Json::Object(&[$(($key, $crate::const_json!($val))),*])
    };
    ($expr:expr) => {$crate::JsonSmuggler::new($expr).to_json()};
}

// Used for automatic type inference
#[doc(hidden)]
pub struct JsonSmuggler<T>(T);

impl<T> JsonSmuggler<T> {
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }
}

macro_rules! smuggle {
    ($name:ident($ty:ty)) => {
        impl JsonSmuggler<$ty> {
            pub const fn to_json(&self) -> Json<'_> {
                Json::$name(self.0)
            }
        }
    };
}

smuggle!(Null(()));
smuggle!(Bool(bool));
smuggle!(Float(f64));
smuggle!(Int(i64));
smuggle!(Str(&'_ str));

impl<'a> JsonSmuggler<Json<'a>> {
    pub const fn to_json(&self) -> Json<'a> {
        self.0
    }
}

impl<'a> Index<usize> for Json<'a> {
    type Output = Json<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_idx(index)
    }
}

impl<'a> Index<&'a str> for Json<'a> {
    type Output = Json<'a>;

    fn index(&self, index: &'a str) -> &Self::Output {
        self.get_val(index)
    }
}

impl core::fmt::Debug for Json<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Json::Null(()) => f.write_str("null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Float(fl) => write!(f, "{fl}"),
            Json::Int(i) => write!(f, "{i}"),
            Json::Str(s) => write!(f, "{s:?}"),
            Json::Array(a) => write!(f, "{a:?}"),

            Json::Object(o) => {
                f.write_str("{")?;
                let mut idx = 0;
                while idx < o.len() {
                    let (k, v) = o[idx];
                    write!(f, " {k:?}: {v:?}")?;
                    if idx < o.len() - 1 {
                        f.write_str(",")?;
                    }

                    idx += 1;
                }

                f.write_str(" }")
            }
        }
    }
}
