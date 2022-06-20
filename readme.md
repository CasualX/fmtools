Formatting Tools
================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/fmtools.svg)](https://crates.io/crates/fmtools)
[![docs.rs](https://docs.rs/fmtools/badge.svg)](https://docs.rs/fmtools)
[![Build status](https://github.com/CasualX/fmtools/workflows/CI/badge.svg)](https://github.com/CasualX/fmtools/actions)

Fast, minimal, feature-rich, extended formatting syntax for Rust!

Features include:

* Arbitrary expressions inside the formatting braces
* Generates optimized Rust code at compiletime
* Supports rust-analyzer auto complete, refactoring and more!
* Supports Rust's standard formatting specifiers
* Single package, no proc-macro, no_std compatible, no extra dependencies
* Control flow allows conditional and repeated formatting
* Capture variables by value or by reference
* Escape hatch to inject custom formatting code

In your Cargo.toml, add:

```text
[dependencies]
fmtools = "0.1"
```

Examples
--------

### Basic usage

```rust
fn basic_usage() -> String {
	let name = "World";

	fmtools::format!("Hello "{name}"!")
}

assert_eq!(basic_usage(), "Hello World!");
```

The value arguments can be arbitrary expressions.
They are inlined in the formatting braces and are outside the string literals.

### Formatting specifiers

```rust
fn formatting_specifiers() -> String {
	let value = 42;

	fmtools::format!("hex("{value}") = "{value:#x})
}

assert_eq!(formatting_specifiers(), "hex(42) = 0x2a");
```

The rules for the specifiers are exactly the same as [the standard library](https://doc.rust-lang.org/std/fmt/index.html) of Rust.

### Let bindings

```rust
fn let_bindings() -> String {
	let base = 52;

	fmtools::format! {
		let value = base - 10;
		"value = "{value}
	}
}

assert_eq!(let_bindings(), "value = 42");
```

Introduce new variable bindings to hold onto temporary values used in the formatting.

### Control flow

```rust
fn control_flow1() -> String {
	let power = 0.5;

	fmtools::format! {
		"At "
		if power >= 1.0 { "full" } else { {power * 100.0:.0}"%" }
		" power"
	}
}

assert_eq!(control_flow1(), "At 50% power");
```

```rust
fn control_flow2() -> String {
	let value = Some(42);

	fmtools::format! {
		"The answer is "
		match value {
			Some(answer) => "Some("{answer}")",
			None => "None",
		}
	}
}

assert_eq!(control_flow2(), "The answer is Some(42)");
```

```rust
fn control_flow3() -> String {
	let values = [1, 2, 3, 4, 5];

	fmtools::format! {
		for &val in &values {
			let result = val * 5;
			"* "{val}" x 5 = "{result}"\n"
		}
	}
}

assert_eq!(control_flow3(), "\
	* 1 x 5 = 5\n\
	* 2 x 5 = 10\n\
	* 3 x 5 = 15\n\
	* 4 x 5 = 20\n\
	* 5 x 5 = 25\n");
```

Control flow really shows the added value of the extended formatting syntax.

### Capture by value

```rust
fn capture_by_value() -> String {
	fn inner() -> impl std::fmt::Display {
		let a = 42;
		fmtools::fmt!(move "a = "{a})
	}
	fmtools::format!("{"{inner()}"}")
}

assert_eq!(capture_by_value(), "{a = 42}");
```

The displayable object can own the captured variables with `move` and can be returned from functions.

### Custom formatting

```rust
fn custom_formatting() -> String {
	fmtools::format! {
		"Now entering ["
		|f| f.write_str("custom formatting")?;
		"]"
	}
}

assert_eq!(custom_formatting(), "Now entering [custom formatting]");
```

Closure syntax provides an escape hatch to inject custom code if needed.
The argument's type is [`&mut Formatter`](https://doc.rust-lang.org/std/fmt/struct.Formatter.html).

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
