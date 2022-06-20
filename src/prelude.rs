/*!
Replace the standard formatting macros using [fmt syntax](crate::fmt!).
*/

/// Replaces `print!` using [fmt syntax](crate::fmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! print {
	($($tt:tt)*) => {
		::std::print!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `println!` using [fmt syntax](crate::fmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! println {
	($($tt:tt)*) => {
		::std::print!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `eprint!` using [fmt syntax](crate::fmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprint {
	($($tt:tt)*) => {
		::std::eprint!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `eprintln!` using [fmt syntax](crate::fmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprintln {
	($($tt:tt)*) => {
		::std::eprint!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `write!` using [fmt syntax](crate::fmt!).
#[macro_export]
macro_rules! write {
	($dst:expr, $($tt:tt)*) => {
		::core::write!($dst, "{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `writeln!` using [fmt syntax](crate::fmt!).
#[macro_export]
macro_rules! writeln {
	($dst:expr, $($tt:tt)*) => {
		::core::write!($dst, "{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `format!` using [fmt syntax](crate::fmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! format {
	($($tt:tt)*) => {
		::std::format!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `format_args!` using [fmt syntax](crate::fmt!).
#[macro_export]
macro_rules! format_args {
	($($tt:tt)*) => {
		::core::format_args!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `panic!` using [fmt syntax](crate::fmt!).
#[macro_export]
macro_rules! panic {
	($($tt:tt)*) => {
		::core::panic!("{}", $crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		}))
	};
}

#[cfg(test)]
#[test]
fn test_prelude() {
	use std::fmt::Write;
	print!("print");
	println!("println");
	eprint!("eprint");
	eprintln!("eprintln");
	let mut s = format!("format");
	let _ = write!(s, "write");
	let _ = writeln!(s, "writeln");
	// tpanic!("panic");
}
