use core::marker::PhantomData;

pub struct FnMutant<In, Extra, Out, F: for<'a> Fn(In, Extra) -> Out> {
    pub f: F,
    phantom_in: PhantomData<In>,
    phantom_extra: PhantomData<Extra>,
    phantom_out: PhantomData<Out>,
}

impl<In, Extra, Out, F: for<'a> Fn(In, Extra) -> Out> FnMutant<In, Extra, Out, F> {
    pub fn new(f: F) -> Self {
        Self {
            f,
            phantom_in: PhantomData,
            phantom_extra: PhantomData,
            phantom_out: PhantomData,
        }
    }
}

pub fn mutant<In, Extra, Out, F: for<'a> Fn(In, Extra) -> Out>(
    f: F,
) -> FnMutant<In, Extra, Out, F> {
    FnMutant::new(f)
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;
    use crate::FnMutant;

    fn incr(x: &mut i32, amount: i32) {
        // Tried doing some thread_local stuff here, but couldn't get that to work (thankfully).
        *x += amount
    }

    fn decr(x: &mut i32, amount: i32) {
        *x -= amount
    }


    fn helper(fps: &[FnMutant<&mut i32, i32, (), &dyn for<'b> Fn(&mut i32, i32)>],
              xs: &mut [i32], expect: Vec<i32>) {
            let len = xs.len();
            for i in 0..len * 5 {
                for fp in fps {
                    (fp.f)(xs.get_mut(i % len).unwrap(), 1);
                }
            }
            assert_eq!(xs, expect.as_slice());
    }

    #[test]
    fn it_works() {
        use super::*;
        let fps: [FnMutant<&mut i32, i32, (), &dyn for<'b> Fn(&mut i32, i32)>; 3] =
            [mutant(&incr), mutant(&decr), mutant(&incr)];
        {
            let mut xs = [0, 1, 2, 3, 4, 5];
            let expect = Vec::from_iter(xs.iter().map(|x| x + 5));
            helper(&fps, &mut xs, expect.clone());
            assert_eq!(xs, expect.as_slice());
            drop(xs);
        }
        {
            let mut xs = [0, 1, 2, 3, 4, 5];
            let expect = Vec::from_iter(xs.iter().map(|x| x + 5));
            helper(&fps, &mut xs, expect.clone());
            assert_eq!(xs, expect.as_slice());
            drop(xs);
        }
    }
}
