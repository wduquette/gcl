//! Public Type Declarations
//!
//! This module defines a number of types used throughout Molt's public API.
//!
//! The most important types are [`Value`], the type of data values in the Molt
//! language, and [`MoltResult`], Molt's standard `Result<T,E>` type.  `MoltResult`
//! is an alias for `Result<Value,Exception>`, where [`Exception`] contains the data
//! relating to an exceptional return from a script.  The heart of `Exception` is the
//! [`ResultCode`], which represents all of the ways a Molt script might return early:
//! errors, explicit returns, breaks, and continues.
//!
//! [`MoltInt`], [`MoltFloat`], [`MoltList`], and [`MoltDict`] are simple type aliases
//! defining Molt's internal representation for integers, floats, and TCL lists and
//! dictionaries.
//!
//! [`MoltResult`]: type.MoltResult.html
//! [`Exception`]: type.Exception.html
//! [`MoltInt`]: type.MoltInt.html
//! [`MoltFloat`]: type.MoltFloat.html
//! [`MoltList`]: type.MoltList.html
//! [`MoltDict`]: type.MoltDict.html
//! [`ResultCode`]: enum.ResultCode.html
//! [`Value`]: ../value/index.html
//! [`interp`]: interp/index.html

use crate::interp::Interp;
pub use crate::value::Value;
use indexmap::IndexMap;

// Molt Numeric Types

/// The standard integer type for Molt code.
///
/// The interpreter uses this type internally for all Molt integer values.
/// The primary reason for defining this as a type alias is future-proofing: at
/// some point we may wish to replace `MoltInt` with a more powerful type that
/// supports BigNums, or switch to `i128`.
pub type MoltInt = i64;

/// The standard floating point type for Molt code.
///
/// The interpreter uses this type internally for all Molt floating-point values.
/// The primary reason for defining this as a type alias is future-proofing: at
/// some point we may wish to replace `MoltFloat` with `f128`.
pub type MoltFloat = f64;

/// The standard list type for Molt code.
///
/// Lists are an important data structure, both in Molt code proper and in Rust code
/// that implements and works with Molt commands.  A list is a vector of `Value`s.
pub type MoltList = Vec<Value>;

/// The standard dictionary type for Molt code.
///
/// A dictionary is a mapping from `Value` to `Value` that preserves the key insertion
/// order.
pub type MoltDict = IndexMap<Value, Value>;

/// The standard `Result<T,E>` type for Molt code.
///
/// This is the return value of all Molt commands, and the most common return value
/// throughout the Molt code base.  Every Molt command returns a [`Value`] on success;
/// if the command has no explicit return value, it returns the empty `Value`, a `Value`
/// whose string representation is the empty string.
///
/// A Molt command returns an [`Exception`] whenever the calling Molt script should
/// return early: on error, when returning an explicit result via the `return` command,
/// or when breaking out of a loop via the `break` or `continue` commands.  The precise
/// nature of the return is indicated by the [`Exception`]'s [`ResultCode`].
///
/// Many of the functions in Molt's Rust API also return `MoltResult`, for easy use within
/// Molt command definitions.
///
/// [`Exception`]: struct.Exception.html
/// [`ResultCode`]: enum.ResultCode.html
/// [`Value`]: ../value/index.html
pub type MoltResult = Result<Value, Exception>;

/// This enum represents the different kinds of [`Exception`] that result from
/// evaluating a Molt script.
///
/// Client code will usually see only the `Error` code; the others will most often be caught
/// and handled within the interpreter.  However, client code may explicitly catch and handle
/// the `Return`, `Break`, and `Continue` codes at both the Rust and the TCL level
/// (see the `catch` command) in order to implement application-specific control structures.
///
/// # Future Work
///
/// * Standard TCL allows for an arbitrary number of result codes, which in turn allows the
///   application to define an arbitrary number of new kinds of control structures that are
///   distinct from the standard ones.  This type provides the `Other` code for this
///   purpose; however, there is as yet no support for it in either the Rust or TCL APIs.
///
/// [`Exception`]: struct.Exception.html

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResultCode {
    /// Experimental; not in use yet.
    Okay,

    /// A Molt error.  The `Exception::value` is the error message for display to the user.
    Error,

    /// An explicit return from a Molt procedure.  The `Exception::value` is the returned
    /// value, or the empty value if `return` was called without a return value.  This result
    /// will bubble up until it reaches the top-level of the enclosing procedure, which will
    /// then return the value as a normal `Ok` result.  If it is received when evaluating an
    /// arbitrary script, i.e., if `return` is called outside of any procedure, the
    /// interpreter will convert it into a normal `Ok` result.
    Return,

    /// A `break` in a Molt loop.  It will break out of the inmost enclosing loop in the usual
    /// way.  If it is returned outside a loop (or some user-defined control structure that
    /// supports `break`), the interpreter will convert it into an error.
    Break,

    /// A `continue` in a Molt loop.  Execution will continue with the next iteration of
    /// the inmost enclosing loop in the usual way.  If it is returned outside a loop (or
    /// some user-defined control structure that supports `break`), the interpreter will
    /// convert it into an error.
    Continue,

    /// Experimental; not in use yet.
    Other(MoltInt)
}

/// This enum represents the exceptional results of evaluating a Molt script, as
/// used in [`MoltResult`].  It is often used as the `Err` type for other
/// functions in the Molt API, so that these functions can easily return errors when used
/// in the definition of Molt commands.
///
/// A Molt script can return a normal result, as indicated by [`MoltResult`]'s `Ok`
/// variant, or it can return one of a number of exceptional results, which
/// will bubble up the call stack in the usual way until caught.  The different kinds of
/// exceptional result are defined by the [`ResultCode`] enum.
///
/// [`ResultCode`]: enum.ResultCode.html
/// [`MoltResult`]: type.MoltResult.html

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Exception {
    /// The kind of exception
    code: ResultCode,

    /// The result value
    value: Value,
}

impl Exception {
    /// Creates an `Error` exception with the given error message.
    pub fn molt_err(msg: Value) -> Self {
        Self {
            code: ResultCode::Error,
            value: msg,
        }
    }

    /// Creates a `Return` exception, with the given return value.  Return `Value::empty()`
    /// if there is no specific result.
    pub fn molt_return(msg: Value) -> Self {
        Self {
            code: ResultCode::Return,
            value: msg,
        }
    }

    /// Creates a `Break` exception.
    pub fn molt_break() -> Self {
        Self {
            code: ResultCode::Break,
            value: Value::empty(),
        }
    }

    /// Creates a `Continue` exception.
    pub fn molt_continue() -> Self {
        Self {
            code: ResultCode::Continue,
            value: Value::empty(),
        }
    }

    /// Gets the exception's result code.
    pub fn code(&self) -> ResultCode {
        self.code
    }

    /// Gets the exception's value, i.e., the explicit return value or the error message.
    pub fn value(&self) -> Value {
        self.value.clone()
    }
}


/// A unique identifier, used to identify cached context data within a given
/// interpreter.  For more information see the discussion of command definition
/// and the context cache in [The Molt Book] and the [`interp`] module.
///
/// [The Molt Book]: https://wduquette.github.io/molt/
/// [`interp`]: ../interp/index.html

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct ContextID(pub(crate) u64);

/// A function used to implement a binary Molt command. For more information see the
/// discussion of command definition in [The Molt Book] and the [`interp`] module.
///
/// The command may retrieve its application context from the [`interp`]'s context cache
/// if it was defined with a [`ContextID`].
///
/// The command function receives the interpreter, the context ID, and a slice
/// representing the command and its arguments.
///
/// [The Molt Book]: https://wduquette.github.io/molt/
/// [`interp`]: ../interp/index.html
/// [`ContextID`]: struct.ContextID.html
pub type CommandFunc = fn(&mut Interp, ContextID, &[Value]) -> MoltResult;

/// A Molt command that has subcommands is called an _ensemble_ command.  In Rust code,
/// the ensemble is defined as an array of `Subcommand` structs, each one mapping from
/// a subcommand name to the implementing [`CommandFunc`].  For more information,
/// see the discussion of command definition in [The Molt Book] and the [`interp`] module.
///
/// The tuple fields are the subcommand's name and implementing [`CommandFunc`].
///
/// [The Molt Book]: https://wduquette.github.io/molt/
/// [`interp`]: ../interp/index.html
/// [`CommandFunc`]: type.CommandFunc.html
pub struct Subcommand(pub &'static str, pub CommandFunc);

impl Subcommand {
    /// Looks up a subcommand of an ensemble command by name in a table,
    /// returning the usual error if it can't be found.  It is up to the
    /// ensemble command to call the returned subcommand with the
    /// appropriate arguments.  See the implementation of the `info`
    /// command for an example.
    ///
    /// # TCL Notes
    ///
    /// * In standard TCL, subcommand lookups accept any unambiguous prefix of the
    ///   subcommand name, as a convenience for interactive use.  Molt does not, as it
    ///   is confusing when used in scripts.
    pub fn find<'a>(
        ensemble: &'a [Subcommand],
        sub_name: &str,
    ) -> Result<&'a Subcommand, Exception> {
        for subcmd in ensemble {
            if subcmd.0 == sub_name {
                return Ok(subcmd);
            }
        }

        let mut names = String::new();
        names.push_str(ensemble[0].0);
        let last = ensemble.len() - 1;

        if ensemble.len() > 1 {
            names.push_str(", ");
        }

        if ensemble.len() > 2 {
            let vec: Vec<&str> = ensemble[1..last].iter().map(|x| x.0).collect();
            names.push_str(&vec.join(", "));
        }

        if ensemble.len() > 1 {
            names.push_str(", or ");
            names.push_str(ensemble[last].0);
        }

        molt_err!(
            "unknown or ambiguous subcommand \"{}\": must be {}",
            sub_name,
            &names
        )
    }
}

/// In TCL, variable references have two forms.  A string like "_some_var_(_some_index_)" is
/// the name of an array element; any other string is the name of a scalar variable.  This
/// struct is used when parsing variable references.  The `name` is the variable name proper;
/// the `index` is either `None` for scalar variables or `Some(String)` for array elements.
///
/// The Molt [`interp`]'s variable access API usually handles this automatically.  Should a
/// command need to distinguish between the two cases it can do so by using the
/// the [`Value`] struct's `Value::as_var_name` method.
///
/// [`Value`]: ../value/index.html
/// [`interp`]: ../interp/index.html
#[derive(Debug, Eq, PartialEq)]
pub struct VarName {
    name: String,
    index: Option<String>,
}

impl VarName {
    /// Creates a scalar `VarName` given the variable's name.
    pub fn scalar(name: String) -> Self {
        Self { name, index: None }
    }

    /// Creates an array element `VarName` given the element's variable name and index string.
    pub fn array(name: String, index: String) -> Self {
        Self {
            name,
            index: Some(index),
        }
    }

    /// Returns the parsed variable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parsed array index, if any.
    pub fn index(&self) -> Option<&str> {
        self.index.as_ref().map(|x| &**x)
    }
}
