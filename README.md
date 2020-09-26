This (experimental) crate declares a single type `FnMutant`,

`FnMutant` has a field `f: for<'a> Fn(X) -> Y`,
which acts much like a `fn(X) -> Y` function pointer, but is a `Trait` rather than a type.
As such it can take generic parameters.

This specifies that the `Fn` must be valid for all lifetimes, even those less than the lifetime of `f`.
any references given to f, through `X`, cannot not be captured by `f`.

This results in a function that can be called multiple times (unlike FnOnce),
and can not capture the unique `&mut` borrows (unlike FnMut).

To explain why this type is not a simple type alias for `for<'a> Fn(x) -> Y`,
we can add the type alias, `FnMutantGoodError` below,
And the resulting compiler error from some changes in the tests.

`pub type FnMutantGoodError<'a, In, Extra, Out, F=&'a dyn for<'b> Fn(In,Extra) -> Out> = FnMutant<In, Extra, Out, F>;`

Specifying the default value of F for both helper and fps, works fine,
However if you specify it in one place:
`fn helper(fps: &[FnMutantGoodError<&mut i32, i32, (), &dyn for<'b> Fn(&mut i32, i32)>], ...`

And leave as the default, in the caller:

 `let fps: [FnMutantGoodError<&mut i32, i32, ()>; 3] = ...`

You will see a nice compilation error:
```
--> src/lib.rs:91:20
    |
 91 |             helper(&fps, &mut xs, expect.clone());
    |                    ^^^^ one type is more general than the other
    |
    = note: expected struct `FnMutant<&mut i32, _, _, &dyn for<'r> std::ops::Fn(&'r mut i32, i32)>`
               found struct `FnMutant<&mut i32, _, _, &dyn std::ops::Fn(&mut i32, i32)>`
```

The reason is that aliases do not check bounds, thus current usage of the type is not pretty, since there is no way to specify default type parameters.
