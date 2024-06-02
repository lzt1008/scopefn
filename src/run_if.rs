use either::Either::{self, Left, Right};

pub trait RunIf: Sized {
    #[inline(always)]
    fn run_if<R>(self, condition: bool, block: impl FnOnce(Self) -> R) -> Either<Self, R> {
        if condition {
            Right(block(self))
        } else {
            Left(self)
        }
    }

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
