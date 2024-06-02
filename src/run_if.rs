use either::Either::{self, Left, Right};

pub trait RunIf: Sized {
    /// Runs the block if the condition is true, otherwise returns self.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::RunIf;
    ///
    /// let x = 42;
    ///
    /// let y = x.run_if(true, |x| x + 1);
    ///
    /// assert_eq!(y, 43);
    #[inline(always)]
    fn run_if<R>(self, condition: bool, block: impl FnOnce(Self) -> R) -> Either<Self, R> {
        if condition {
            Right(block(self))
        } else {
            Left(self)
        }
    }

    /// Runs the block if the predicate is true, otherwise returns self.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::RunIf;
    ///
    /// let x = 42;
    ///
    /// let y = x.run_if_with(|x| *x == 42, |x| x + 1);
    ///
    /// assert_eq!(y, 43);
    /// ```
    #[inline(always)]
    fn run_if_with<P, F, R>(self, predicate: P, block: F) -> Either<Self, R>
    where
        P: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> R,
    {
        if predicate(&self) {
            Right(block(self))
        } else {
            Left(self)
        }
    }
}

impl<T: Sized> RunIf for T {}
