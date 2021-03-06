# Molt -- More Or Less TCL

![Rust](https://github.com/wduquette/molt/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/molt.svg)](https://crates.io/crates/molt)

![MoltLogo.png](MoltLogo.png)

Molt is a minimal implementation of the TCL language for embedding in Rust apps and for
scripting Rust libraries.  See [The Molt Book](https://wduquette.github.io/molt) for details
and user documentation, and [docs.rs/molt](https://docs.rs/molt) for the Rust API.

## Mazegen: An Example

See [Mazegen](https://github.com/wduquette/mazegen) as an example of using Molt to provide
a TCL interface for real Rust APIs.  Mazegen is a collection of maze generation code and
related infrastructure, with a Molt interface as an aid for debugging and
experimentation.  Among other things, Mazegen includes:

* A simple binding to (a small part of) the
  [**rand::thread_rng**](https://crates.io/crates/rand) crate
* The smallest beginning of a binding to the [**image**](https://crates.io/crates/image) crate
  (more to come)

## New in Molt 0.3.1

* `molt_throw!` macro
* New `Exception` methods
* Access to environment variables via the env() variable
* Additional TCL commands

See the
[Annotated Change Log](https://wduquette.github.io/molt/changes.md) in the Molt Book for
the complete list of new features by version.

## Coming Attractions

At this point Molt is capable and robust enough for real work, though the Rust-level API is
not yet completely stable.  Standard Rust `0.y.z` semantic versioning applies: ".y" changes
can break the Rust-level API, ".z" changes will not.

*   Additional TCL commands
*   Testing improvements
*   Documentation improvements
*   Feature: Regex and Glob pattern matching by Molt commands

## Why Molt Exists

Using Molt, you can:

*   Create a shell interpreter for scripting and interactive testing of your Rust crates.
*   Provide your Rust applications with an interactive REPL for debugging and
    administration.
*   Extend your Rust application with scripts provided at compile-time or at run-time.
*   Allow your users to script your applications and libraries.

See the [`molt-sample` repo](https://github.com/wduquette/molt-sample) for a sample Molt client
skeleton.

## Molt and Standard TCL

Molt is intended to be lightweight and require minimal dependencies, so that it can be added
to any project without greatly increasing its footprint.  (At present, the core
language is a single library create with no dependencies at all!)  As such, it does not provide
all of the features of Standard TCL (e.g., TCL 8.6).

At the same time, Molt's implementation of TCL should be consistent with TCL 8.6 so far as it
goes.  Some archaic commands and command features are omitted; some changes
are made so Molt works better in the Rust ecosystem. (E.g., Molt's notion of whitespace is
the same as Rust's.) All liens against Standard TCL are documented in
the [The Molt Book](https://wduquette.github.io/molt).

No effort has been made to make the Rust-level API for extending Molt in Rust look like
Standard TCL's C API; rather, the goal is to make the Rust-level API as simple and ergonomic
as possible. **Note**: A big part of this effort is defining and refining the Rust API used
to interact with and extend the interpreter. If you have comments or suggestions for
improvement, please contact me or write an issue!

## Building and Installation

The easiest approach is to get the latest Molt through `crates.io`.  Look for the
`molt`, `molt-shell`, and `molt-app` crates, or add them to your dependencies list
in `cargo.toml`.

To build Molt:

*   Install the latest stable version of Rust (1.38.0 at time of writing)
*   Clone this repository
*   To build:

```
$ cd .../molt
$ cargo build
```

* To run the interactive shell

```
$ cargo run shell
```

* To run just the language test suite

```
$ cargo run test molt/tests/all.tcl
```

Since Molt 0.2.0 the language tests are also run by `cargo test`; however, it's much easier to
see the output of the individual tests using the above command.

## Dependencies

At present, the only dependency required by the Molt core is
[indexmap](https://docs.rs/indexmap/1.3.0/indexmap/).

## Acknowledgements

I've gotten help from many people in this endeavor; here's a (necessarily partial) list.

* Mary Duquette, for the Molt logo
* Jonathan Castello, for general Rust info
* Kevin Kenny, for help with TCL numerics and general encouragement
* Don Porter, for help with TCL parsing
* rfdonnelly, for the crates.io badge, etc.
* Coleman McFarland, for improvements to `molt_shell::repl`.
* dbohdan, for TCL command implementations and advice.
* Various folks from users.rust-lang.org who have answered my questions:
    * Krishna Sannasi, for help getting `Value` to work with arbitrary user data types
    * Yandros, for pointing me at `OnceCell` and `UnsafeCell`.
    * jethrogb, for help on how to use `Ref::map` to return a `Ref<T>` of a component deep within
      a `RefCell<S>` from a function.  (Mind you, once I got it working and gave it a try I
      tore it out again, because of `BorrowMutError` panics.  But I had to try it.)
