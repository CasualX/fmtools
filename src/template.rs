/// Extended formatting syntax.
///
/// Returns a displayable object which can be formatted with `{}`.
///
/// # Examples
///
/// ### Basic usage
///
/// ```
/// let name = "World";
///
/// # let s =
/// fmtools::fmt!("Hello "{name}"!")
/// # .to_string();
/// # assert_eq!(s, "Hello World!");
/// ```
///
/// The resulting string is `Hello World!`.
///
/// The value arguments can be arbitrary expressions.
/// They are inlined in the formatting braces and are outside the string literals.
///
/// ### Formatting specifiers
///
/// ```
/// let value = 42;
///
/// # let s =
/// fmtools::fmt!("hex("{value}") = "{value:#x})
/// # .to_string();
/// # assert_eq!(s, "hex(42) = 0x2a");
/// ```
///
/// The resulting string is `hex(42) = 0x2a`.
///
/// The rules for the specifiers are exactly the same as Rust's [standard formatting syntax](std::fmt).
///
/// ### Let bindings
///
/// ```
/// let base = 52;
///
/// # let s =
/// fmtools::fmt! {
/// 	let value = base - 10;
/// 	"value = "{value}
/// }
/// # .to_string();
/// # assert_eq!(s, "value = 42");
/// ```
///
/// The resulting string is `value = 42`.
///
/// ### Control flow
///
/// ```
/// let power = 0.5;
///
/// # let s =
/// fmtools::fmt! {
/// 	"At "
/// 	if power >= 1.0 { "full" }
/// 	else { {power * 100.0:.0}"%" }
/// 	" power"
/// }
/// # .to_string();
/// # assert_eq!(s, "At 50% power");
/// ```
///
/// The resulting string is `At 50% power`.
///
/// ```
/// let value = Some(42);
///
/// # let s =
/// fmtools::fmt! {
/// 	"The answer is "
/// 	match value {
/// 		Some(answer) => "Some("{answer}")",
/// 		None => "None",
/// 	}
/// }
/// # .to_string();
/// # assert_eq!(s, "The answer is Some(42)");
/// ```
///
/// The resulting string is `The answer is Some(42)`.
///
/// ```
/// let values = [1, 2, 3, 4, 5];
///
/// # let s =
/// fmtools::fmt! {
/// 	for &val in &values {
/// 		let result = val * 5;
/// 		"* "{val}" x 5 = "{result}"\n"
/// 	}
/// }
/// # .to_string();
/// # assert_eq!(s, "* 1 x 5 = 5\n* 2 x 5 = 10\n* 3 x 5 = 15\n* 4 x 5 = 20\n* 5 x 5 = 25\n");
/// ```
///
/// The resulting string is:
///
/// ```text
/// * 1 x 5 = 5
/// * 2 x 5 = 10
/// * 3 x 5 = 15
/// * 4 x 5 = 20
/// * 5 x 5 = 25
/// ```
///
/// Control flow really shows the added value of the extended formatting syntax.
///
/// ### Capture by value
///
/// ```
/// fn inner() -> impl std::fmt::Display {
/// 	let a = 42;
/// 	fmtools::fmt!(move "a = "{a})
/// }
/// # let s =
/// fmtools::fmt!(move "{"{inner()}"}")
/// # .to_string();
/// # assert_eq!(s, "{a = 42}");
/// ```
///
/// The resulting string is `{a = 42}`.
///
/// The displayable object can own the captured variables with `move` and can be returned from functions.
///
/// ### Escape hatch
///
/// ```
/// # let s =
/// fmtools::fmt! {
/// 	"Now entering ["
/// 	|f| f.write_str("escape hatch")?;
/// 	"]"
/// }
/// # .to_string();
/// # assert_eq!(s, "Now entering [escape hatch]");
/// ```
///
/// The resulting string is `Now entering [escape hatch]`.
///
/// Closure syntax provides an escape hatch to inject code if needed.
/// The argument's type is [`&mut Formatter`](std::fmt::Formatter).
#[macro_export]
macro_rules! fmt {
	(move $($tt:tt)*) => {
		$crate::fmt(move |_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		})
	};
	($($tt:tt)*) => {
		$crate::fmt(|_f| {
			$crate::__fmt!{_f $($tt)*}
			Ok(())
		})
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __fmt {
	// text
	($f:ident $text1:literal $text2:literal $($tail:tt)*) => {
		$crate::__fmt!{$f @concat($text1, $text2) $($tail)*}
	};
	($f:ident $text:literal $($tail:tt)*) => {
		$f.write_str($crate::obfstr!(concat!($text)))?;
		$crate::__fmt!{$f $($tail)*}
	};
	($f:ident @concat($($texts:literal),+) $text:literal $($tail:tt)*) => {
		$crate::__fmt!{$f @concat($($texts,)+ $text) $($tail)*}
	};
	($f:ident @concat($($texts:literal),+) $($tail:tt)*) => {
		$f.write_str($crate::obfstr!(concat!($($texts),+)))?;
		$crate::__fmt!{$f $($tail)*}
	};

	// format
	($f:ident {$($e:tt)*} $($tail:tt)*) => {
		$f.write_fmt($crate::__fmt_format!([] $($e)*))?;
		$crate::__fmt!{$f $($tail)*}
	};

	// escape hatch
	($f:ident |$ff:pat_param| $block:block $($tail:tt)*) => {
		let $ff = &mut *$f;
		$block
		$crate::__fmt!{$f $($tail)*}
	};
	($f:ident |$ff:pat_param| $stmt:stmt; $($tail:tt)*) => {
		let $ff = &mut *$f;
		$stmt
		$crate::__fmt!{$f $($tail)*}
	};

	// let
	($f:ident let $p:pat = $e:expr; $($tail:tt)*) => {
		let $p = $e;
		$crate::__fmt!{$f $($tail)*}
	};

	// if
	($f:ident if $($tail:tt)*) => {
		$crate::__fmt_if!{$f [] if $($tail)*}
	};

	// match
	($f:ident match ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_match!{$f match ($e) {} $($body)*}
		$crate::__fmt!{$f $($tail)*}
	};
	($f:ident match $($tail:tt)*) => {
		$crate::__with_parens!{__fmt! [$f match] () $($tail)*}
	};

	// for
	($f:ident for $p:pat in ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		for $p in $e {
			$crate::__fmt!{$f $($body)*}
		}
		$crate::__fmt!{$f $($tail)*}
	};
	($f:ident for $p:pat in $($tail:tt)*) => {
		$crate::__with_parens!{__fmt! [$f for $p in] () $($tail)*}
	};

	// optimization
	($f:ident ($($tt:tt)*) $($tail:tt)*) => {
		$crate::__fmt!{$f $($tt)*}
		$crate::__fmt!{$f $($tail)*}
	};

	// term
	($f:ident) => {};
}


// Parse the formatting inside formatting braces.
#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_format {
	([$($e:tt)*] : $($tail:tt)*) => {
		$crate::__fmt_expr!([$($e)*] : $($tail)*)
	};
	([$($e:tt)*] ; $($tail:tt)*) => {
		$crate::__fmt_expr!([$($e)*] : $($tail)*)
	};
	([$($e:tt)*] $nom:tt $($tail:tt)*) => {
		$crate::__fmt_format!([$($e)* $nom] $($tail)*)
	};
	([$($e:tt)*]) => {
		$crate::__fmt_expr!([$($e)*])
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_expr {
	([$e:expr]) => {
		::core::format_args!("{}", $e)
	};
	([$e:expr $(, $w:expr)?] $($s:tt)*) => {
		::core::format_args!(concat!("{", $(::core::stringify!($s),)* "}"), $e $(,$w)?)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_if {
	// if let
	($f:ident [$($c:tt)*] if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_if!{$f [$($c)* if let $p = $e { $crate::__fmt!{$f $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] if let $p:pat = $($tail:tt)*) => {
		$crate::__with_parens!{__fmt_if! [$f [$($c)*] if let $p =] () $($tail)*}
	};

	// if
	($f:ident [$($c:tt)*] if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_if!{$f [$($c)* if $e { $crate::__fmt!{$f $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] if $($tail:tt)*) => {
		$crate::__with_parens!{__fmt_if! [$f [$($c)*] if] () $($tail)*}
	};

	// else if let
	($f:ident [$($c:tt)*] else if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_if!{$f [$($c)* else if let $p = $e { $crate::__fmt!{$f $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] else if let $p:pat = $($tail:tt)*) => {
		$crate::__with_parens!{__fmt_if! [$f [$($c)*] else if let $p =] () $($tail)*}
	};

	// else if
	($f:ident [$($c:tt)*] else if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_if!{$f [$($c)* else if $e { $crate::__fmt!{$f $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] else if $($tail:tt)*) => {
		$crate::__with_parens!{__fmt_if! [$f [$($c)*] else if] () $($tail)*}
	};

	// else
	($f:ident [$($c:tt)*] else { $($body:tt)* } $($tail:tt)*) => {
		$($c)*
		else {
			$crate::__fmt!{$f $($body)*}
		}
		$crate::__fmt!{$f $($tail)*}
	};

	// term
	($f:ident [$($c:tt)*] $($tail:tt)*) => {
		$($c)*
		$crate::__fmt!{$f $($tail)*}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __with_parens {
	($next:ident! [$($prefix:tt)*] ($($tt:tt)*) { $($body:tt)* } $($tail:tt)*) => {
		$crate::$next!{$($prefix)* ($($tt)*) { $($body)* } $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] ($($tt:tt)*) $nom:tt $($tail:tt)*) => {
		$crate::__with_parens!{$next! [$($prefix)*] ($($tt)* $nom) $($tail)*}
	};
	// Allows auto complete to work without the following {}
	($next:ident! [$($prefix:tt)*] ($($tt:tt)*)) => {
		$crate::$next!{$($prefix)* ($($tt)*) {}}
		compile_error!(concat!("missing block after expression: ", stringify!($($tt)*)));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __fmt_match {
	($f:ident match ($e:expr) {$($arms:tt)*} $pat:pat $(if $guard:expr)? => { $($body:tt)* }, $($tail:tt)*) => {
		$crate::__fmt_match!{$f match ($e) {$($arms)* $pat $(if $guard)? => { $crate::__fmt!{$f $($body)*} }} $($tail)*}
	};
	($f:ident match ($e:expr) {$($arms:tt)*} $pat:pat $(if $guard:expr)? => { $($body:tt)* } $($tail:tt)*) => {
		$crate::__fmt_match!{$f match ($e) {$($arms)* $pat $(if $guard)? => { $crate::__fmt!{$f $($body)*} }} $($tail)*}
	};
	($f:ident match ($e:expr) {$($arms:tt)*} $pat:pat $(if $guard:expr)? => $($tail:tt)*) => {
		$crate::__until_comma!{__fmt_match! [$f match ($e) {$($arms)*} $pat $(if $guard)? =>] {} $($tail)*}
	};
	($f:ident match ($e:expr) {$($pat:pat $(if $guard:expr)? => $block:block)*}) => {
		match $e {
			$($pat $(if $guard)? => $block)*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __until_comma {
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*} , $($tail:tt)*) => {
		$crate::$next!{$($prefix)* {$($tt)*}, $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*} $nom:tt $($tail:tt)*) => {
		$crate::__until_comma!{$next! [$($prefix)*] {$($tt)* $nom} $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*}) => {
		$crate::$next!{$($prefix)* {$($tt)*}}
	};
}

#[test]
fn tests() {
	#[track_caller]
	fn check(t: impl std::fmt::Display, s: &str) {
		assert_eq!(t.to_string(), s);
	}

	// Basic literals
	check(fmt!("abc"), "abc");
	check(fmt!("abc" "def"), "abcdef");
	check(fmt!("abc" 123 "def" '#' 4_2_0_f32), "abc123def#420");
	check(fmt!(0 {0}), "00");

	// Formatting specifiers
	check(fmt!(let value = 42; "hex("{value}") = "{value:#x}), "hex(42) = 0x2a");
	check(fmt!(let width = 5; "Hello "{"x",width:1$}"!"), "Hello x    !");

	// Code injection
	check(fmt!(|_| let name = "world"; |f| f.write_str("Hello ")?; |f| f.write_str(name)?;), "Hello world");
	check(fmt!(|_| let name = "world"; |f| { f.write_str("Hello ")?; f.write_str(name)?; }), "Hello world");

	// Move ownership
	check(fmt!("{"{fmt!("a = "{42})}"}"), "{a = 42}");
	check(fmt!("{"{{let a = 42; fmt!(move "a = "{a})}}"}"), "{a = 42}");

	// Control flow
	let _ = fmt!(if false {});
	let _ = fmt!(if false {} if true {});
	let _ = fmt!(if false {} else {});
	let _ = fmt!(if false {} else if true {});
	let _ = fmt!(if false {} else if false {} else {});
	let _ = fmt!(if let Some(_) = Some(42) {});
	let _ = fmt!(if let Some(_) = Some(42) {} else {});
	let _ = fmt!(if let Some(_) = Some(42) {} else if let Some(_) = Some(13) {});
	let _ = fmt!(if let Some(_) = Some(42) {} else if let Some(_) = Some(13) {} else {});
	let _ = fmt!(match false { false => {}, true => {}});
	let _ = fmt!(match 0i32 { 0 => "0", 1 => {} _ => {},});
	let _ = fmt!(match false { _ if false => {} _ => {}});
	let _ = fmt!(for _ in 0..4 {});
	let _ = fmt!(for _ in &[1, 2, 3, 4] {});

	// Optimize large fmt invocations
	check(fmt!(
		(0 {1} 2 3 {4} 5 6 {7} 8 9 {0} 1 2 {3} 4 5 6 {7} 8 9 {0} 1 2 {3} 4 5 {6} 7 8 {9} 0 1)
		(0 {1} 2 3 {4} 5 6 {7} 8 9 {0} 1 2 {3} 4 5 6 {7} 8 9 {0} 1 2 {3} 4 5 {6} 7 8 {9} 0 1)
		(0 {1} 2 3 {4} 5 6 {7} 8 9 {0} 1 2 {3} 4 5 6 {7} 8 9 {0} 1 2 {3} 4 5 {6} 7 8 {9} 0 1)
		(0 {1} 2 3 {4} 5 6 {7} 8 9 {0} 1 2 {3} 4 5 6 {7} 8 9 {0} 1 2 {3} 4 5 {6} 7 8 {9} 0 1)
	), concat!("01234567890123456789012345678901", "01234567890123456789012345678901",
		"01234567890123456789012345678901", "01234567890123456789012345678901"));
}
