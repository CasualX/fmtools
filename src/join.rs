use core::fmt;

/// Displays an iterable with given separator between each item.
///
/// ```
/// let result = fmtools::join("--", &[1, 2, 3, 4]).to_string();
/// assert_eq!(result, "1--2--3--4");
/// ```
#[inline]
pub fn join<T>(sep: &'static str, collection: T) -> impl fmt::Display + fmt::Debug where T: IntoIterator, <T as IntoIterator>::Item: fmt::Display, <T as IntoIterator>::IntoIter: Clone {
	let iter = collection.into_iter();
	crate::fmt(move |f| {
		let mut draw = false;
		for item in iter.clone() {
			if sep.len() > 0 {
				if draw {
					f.write_str(sep)?;
				}
				draw = true;
			}
			fmt::Display::fmt(&item, f)?;
		}
		Ok(())
	})
}

/// Joins the arguments in a displayable object.
///
/// ```
/// let result = fmtools::join!("--"; 1, 2.5, true).to_string();
/// assert_eq!(result, "1--2.5--true");
/// ```
///
/// Optionally, the formatting can be specified:
///
/// ```
/// let result = fmtools::join!("--"; 10, 11, 12; "{:#x}").to_string();
/// assert_eq!(result, "0xa--0xb--0xc");
/// ```
///
/// The arguments can be captured by value and returned:
///
/// ```
/// fn inner() -> impl std::fmt::Display {
/// 	let (a, b, c) = (1, 2.5, true);
/// 	return fmtools::join!(move "--"; a, b, c);
/// }
/// let result = inner().to_string();
/// assert_eq!(result, "1--2.5--true");
/// ```
#[macro_export]
macro_rules! join {
	(move $sep:literal; $($e:expr),+) => {
		$crate::fmt(move |f| {
			f.write_fmt($crate::__join!(concat!(), ; $sep; $($e),+; "{}"))
		})
	};
	(move $sep:literal; $($e:expr),+; $s:literal) => {
		$crate::fmt(move |f| {
			f.write_fmt($crate::__join!(concat!(), ; $sep; $($e),+; $s))
		})
	};
	($sep:literal; $($e:expr),+) => {
		$crate::fmt(|f| {
			f.write_fmt($crate::__join!(concat!(), ; $sep; $($e),+; "{}"))
		})
	};
	($sep:literal; $($e:expr),+; $s:literal) => {
		$crate::fmt(|f| {
			f.write_fmt($crate::__join!(concat!(), ; $sep; $($e),+; $s))
		})
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __join {
	(concat!($($fmt:expr,)*), $($arg:expr,)*; $sep:literal; $e:expr; $s:literal) => {
		format_args!(concat!($($fmt,)* $s), $($arg,)* $e)
	};
	(concat!($($fmt:expr,)*), $($arg:expr,)*; $sep:literal; $e:expr, $($tail:expr),+; $s:literal) => {
		$crate::__join!(concat!($($fmt,)* $s, $sep,), $($arg,)* $e,; $sep; $($tail),+; $s)
	};
}

#[test]
fn tests() {
	#[track_caller]
	fn check(f: impl fmt::Display, s: &str) {
		assert_eq!(f.to_string(), s);
	}

	check(join!(" "; 10), "10");
	check(join!(","; 10, 11; "{:#x}"), "0xa,0xb");
	fn inner() -> impl fmt::Display {
		let (a, b) = (10, 11);
		return join!(move " "; a, b);
	}
	check(join!("; "; join!(" "; 'a', 'b'), join!(" "; 'b', 'c'), inner()), "a b; b c; 10 11");
}
