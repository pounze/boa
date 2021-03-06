//! This module implements the global `Symbol` object.
//!
//! The data type symbol is a primitive data type.
//! The `Symbol()` function returns a value of type symbol, has static properties that expose
//! several members of built-in objects, has static methods that expose the global symbol registry,
//! and resembles a built-in object class, but is incomplete as a constructor because it does not
//! support the syntax "`new Symbol()`".
//!
//! Every symbol value returned from `Symbol()` is unique.
//!
//! More information:
//! - [MDN documentation][mdn]
//! - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-symbol-value
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol

#[cfg(test)]
mod tests;

use crate::{
    builtins::{
        object::{
            internal_methods_trait::ObjectInternalMethods, Object, ObjectKind, INSTANCE_PROTOTYPE,
            PROTOTYPE,
        },
        value::{to_value, ResultValue, Value, ValueData},
    },
    exec::Interpreter,
};
use gc::{Gc, GcCell};
use rand::random;

/// Creates Symbol instances.
///
/// Symbol instances are ordinary objects that inherit properties from the Symbol prototype object.
/// Symbol instances have a `[[SymbolData]]` internal slot.
/// The `[[SymbolData]]` internal slot is the Symbol value represented by this Symbol object.
///
/// More information:
/// - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-symbol-description
pub fn call_symbol(_: &Value, args: &[Value], ctx: &mut Interpreter) -> ResultValue {
    // From an implementation and specificaition perspective Symbols are similar to Objects.
    // They have internal slots to hold the SymbolData and Description, they also have methods and a prototype.
    // So we start by creating an Object
    // TODO: Set prototype to Symbol.prototype (by changing to Object::create(), use interpreter to get Symbol.prototype)
    let mut sym_instance = Object::default();
    sym_instance.kind = ObjectKind::Symbol;

    // Set description which should either be undefined or a string
    let desc_string = match args.get(0) {
        Some(value) => to_value(value.to_string()),
        None => Gc::new(ValueData::Undefined),
    };

    sym_instance.set_internal_slot("Description", desc_string);
    sym_instance.set_internal_slot("SymbolData", to_value(random::<i32>()));

    // Set __proto__ internal slot
    let proto = ctx
        .realm
        .global_obj
        .get_field_slice("Symbol")
        .get_field_slice(PROTOTYPE);
    sym_instance.set_internal_slot(INSTANCE_PROTOTYPE, proto);

    Ok(Gc::new(ValueData::Symbol(GcCell::new(sym_instance))))
}

/// `Symbol.prototype.toString()`
///
/// This method returns a string representing the specified `Symbol` object.
///
/// /// More information:
/// - [MDN documentation][mdn]
/// - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-symbol.prototype.tostring
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toString
pub fn to_string(this: &Value, _: &[Value], _: &mut Interpreter) -> ResultValue {
    let s: Value = this.get_internal_slot("Description");
    let full_string = format!(r#"Symbol({})"#, s.to_string());
    Ok(to_value(full_string))
}

/// The `Symbol()` constructor returns a value of type **symbol**.
///
/// It is incomplete as a constructor because it does not support the syntax "`new Symbol()`".
///
/// More information:
/// - [MDN documentation][mdn]
/// - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#sec-symbol-constructor
/// [mdn]:
pub fn create_constructor(global: &Value) -> Value {
    // Create Symbol constructor (or function in Symbol's case)
    let mut symbol_constructor = Object::default();
    symbol_constructor.set_internal_method("call", call_symbol);

    // Create prototype
    let mut symbol_prototype = Object::default();

    // Symbol.prototype[[Prototype]] points to Object.prototype
    // Symbol Constructor -> Symbol Prototype -> Object Prototype
    let object_prototype = global.get_field_slice("Object").get_field_slice(PROTOTYPE);
    symbol_prototype.set_internal_slot(INSTANCE_PROTOTYPE, object_prototype);
    symbol_prototype.set_method("toString", to_string);

    let symbol_prototype_val = to_value(symbol_prototype);

    let symbol_constructor_value = to_value(symbol_constructor);
    symbol_prototype_val.set_field_slice("construcotor", symbol_constructor_value.clone());
    symbol_constructor_value.set_field_slice(PROTOTYPE, symbol_prototype_val);

    symbol_constructor_value
}
