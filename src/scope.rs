pub trait Apply: Sized {
    /// Calls the specified function `block` with reference of Self and returns Self.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::Apply;
    ///
    /// let mut list = vec![1, 2, 3];
    /// list.apply(|v| {
    ///    println!("{:?}", v);
    /// });
    ///
    /// ```
    #[inline(always)]
    fn apply(self, block: impl FnOnce(&Self)) -> Self {
        block(&self);
        self
    }

    /// Calls the specified function `block` with mutable reference of Self and returns Self.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::Apply;
    ///
    /// let list = vec![1, 2, 3].apply_mut(|v| v.push(4));
    ///
    /// assert_eq!(list, vec![1, 2, 3, 4]);
    #[inline(always)]
    fn apply_mut(mut self, block: impl FnOnce(&mut Self)) -> Self {
        block(&mut self);
        self
    }
}

impl<T: Sized> Apply for T {}

pub trait Run: Sized {
    /// Calls the specified function `block` with Self and returns the result of the function.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::Run;
    ///
    /// let result = 10.run(|x| x * 2);
    /// assert_eq!(result, 20);
    ///
    /// let result = ["Hello", "World"].run(|x| x.join(" "));
    /// assert_eq!(result, "Hello World");
    /// ```
    #[inline(always)]
    fn run<R>(self, block: impl FnOnce(Self) -> R) -> R {
        block(self)
    }
}

impl<T: Sized> Run for T {}

pub trait TakeIf: Sized {
    /// Returns `Some(self)` if the predicate is true, otherwise `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::TakeIf;
    ///
    /// let x = 42;
    ///
    /// let y = x.take_if(|x| *x == 42);
    ///
    /// assert_eq!(y, Some(42));
    /// ```
    #[inline(always)]
    fn take_if(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }

    /// Returns `Some(self)` if the predicate is false, otherwise `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use scopefn::TakeIf;
    ///
    /// let x = 42;
    ///
    /// let y = x.take_unless(|x| *x != 42);
    ///
    /// assert_eq!(y, Some(42));
    #[inline(always)]
    fn take_unless(self, predicate: impl FnOnce(&Self) -> bool) -> Option<Self> {
        self.take_if(
            #[inline(always)]
            |x| !predicate(x),
        )
    }
}

impl<T: Sized> TakeIf for T {}
