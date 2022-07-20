/*!
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

See [fmt!] for more information.
*/

#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::fmt as core_fmt;

mod template;
mod prelude;

mod join;
pub use self::join::*;

// Formattable object holder.
//
// Exported but hidden to support `Copy` + `Clone` if the closure implements these traits.
// This is otherwise not supported by existential impl trait types.
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct fmt<F: Fn(&mut core_fmt::Formatter) -> core_fmt::Result> {
	closure: F,
}
impl<F: Fn(&mut core_fmt::Formatter) -> core_fmt::Result> core_fmt::Display for fmt<F> {
	fn fmt(&self, f: &mut core_fmt::Formatter) -> core_fmt::Result {
		(self.closure)(f)
	}
}
impl<F: Fn(&mut core_fmt::Formatter) -> core_fmt::Result> core_fmt::Debug for fmt<F> {
	fn fmt(&self, f: &mut core_fmt::Formatter) -> core_fmt::Result {
		(self.closure)(f)
	}
}

/// Returns a displayable object using the closure argument as its implementation.
///
/// ```
/// let s = fmtools::fmt(|f| {
/// 	f.write_str("display")
/// });
/// assert_eq!(s.to_string(), "display");
/// ```
///
/// This is useful to insert ad-hoc formatting code:
///
/// ```
/// println!("Hello {}!", fmtools::fmt(|f| {
/// 	f.write_str("world")
/// }));
/// // Prints `Hello world!`
/// ```
pub fn fmt<F: Fn(&mut core_fmt::Formatter) -> core_fmt::Result>(closure: F) -> fmt<F> {
	fmt { closure }
}

#[cfg(feature = "obfstr")]
#[doc(hidden)]
pub use obfstr::obfstr;

#[cfg(not(feature = "obfstr"))]
#[doc(hidden)]
#[macro_export]
macro_rules! obfstr {
	($s:expr) => { $s };
}

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}
