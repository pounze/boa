//! This module implements the global `JSON` object.
//!
//! The `JSON` object contains methods for parsing [JavaScript Object Notation (JSON)][spec]
//! and converting values to JSON. It can't be called or constructed, and aside from its
//! two method properties, it has no interesting functionality of its own.
//!
//! More information:
//!  - [ECMAScript reference][spec]
//!  - [MDN documentation][mdn]
//!  - [JSON specification][json]
//!
//! [spec]: https://tc39.es/ecma262/#sec-json
//! [json]: https://www.json.org/json-en.html
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON

use crate::builtins::function::NativeFunctionData;
use crate::builtins::value::{to_value, ResultValue, Value, ValueData};
use crate::exec::Interpreter;
use serde_json::{self, Value as JSONValue};

#[cfg(test)]
mod tests;

/// `JSON.parse( text[, reviver] )`
///
/// This `JSON` method parses a JSON string, constructing the JavaScript value or object described by the string.
///
/// An optional `reviver` function can be provided to perform a transformation on the resulting object before it is returned.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-json.parse
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse
// TODO: implement optional revever argument.
pub fn parse(_: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    match serde_json::from_str::<JSONValue>(
        &args
            .get(0)
            .expect("cannot get argument for JSON.parse")
            .clone()
            .to_string(),
    ) {
        Ok(json) => Ok(to_value(json)),
        Err(err) => Err(to_value(err.to_string())),
    }
}

/// `JSON.stringify( value[, replacer[, space]] )`
///
/// This `JSON` method converts a JavaScript object or value to a JSON string.
///
/// This medhod optionally replaces values if a `replacer` function is specified or
/// optionally including only the specified properties if a replacer array is specified.
///
/// An optional `space` argument can be supplied of type `String` or `Number` that's used to insert
/// white space into the output JSON string for readability purposes.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-json.stringify
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify
pub fn stringify(_: &Value, args: &[Value], _: &mut Interpreter) -> ResultValue {
    let obj = args.get(0).expect("cannot get argument for JSON.stringify");
    let json = obj.to_json().to_string();
    Ok(to_value(json))
}

/// Create a new `JSON` object.
pub fn create_constructor(global: &Value) -> Value {
    let json = ValueData::new_obj(Some(global));

    make_builtin_fn!(parse, named "parse", with length 2, of json);
    make_builtin_fn!(stringify, named "stringify", with length 3, of json);

    to_value(json)
}
