This (experimental) crate declares a single type `FnMutant`,

`FnMutant` has a field `f: for<'a> Fn(X) -> Y`, 
This specifies that the closure must be valid for all lifetimes,
thus any references given to f, through `X`, must not be captured by the closure.

This results in a function that can be called multiple times (unlike FnOnce),
and does not capture the unique borrow (unlike FnMut).
Because after being called, given an `&mut T`, the `&mut T` either been returned or have gone out of scope.
