//! Builtins live here, such as Object, String, Math etc

/// Macro to create a new member function of a prototype.
///
/// If no length is provided, the length will be set to 0.
macro_rules! make_builtin_fn {
    ($fn:ident, named $name:expr, with length $l:tt, of $p:ident) => {
        let $fn = to_value($fn as NativeFunctionData);
        $fn.set_field_slice("length", to_value($l));
        $p.set_field_slice($name, $fn);
    };
    ($fn:ident, named $name:expr, of $p:ident) => {
        make_builtin_fn!($fn, named $name, with length 0, of $p);
    };
}

pub mod array;
pub mod boolean;
pub mod console;
pub mod error;
pub mod function;
pub mod json;
pub mod math;
pub mod number;
pub mod object;
pub mod property;
pub mod regexp;
pub mod string;
pub mod symbol;
pub mod value;
