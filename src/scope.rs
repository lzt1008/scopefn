pub trait Apply: Sized {
    #[inline(always)]
    fn apply(self, block: impl FnOnce(&Self)) -> Self {
        block(&self);
        self
    }

    #[inline(always)]
    fn apply_mut(mut self, block: impl FnOnce(&mut Self)) -> Self {
        block(&mut self);
        self
    }
}

impl<T: Sized> Apply for T {}

pub trait Run: Sized {
    #[inline(always)]
    fn run<R>(self, block: impl FnOnce(Self) -> R) -> R {
        block(self)
    }
}

impl<T: Sized> Run for T {}

pub trait TakeIf: Sized {
    #[inline(always)]
    fn take_if(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }

    #[inline(always)]
    fn take_unless(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        self.take_if(
            #[inline(always)]
            |x| !predicate(x),
        )
    }
}

impl<T: Sized> TakeIf for T {}
