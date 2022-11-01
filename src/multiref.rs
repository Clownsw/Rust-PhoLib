use std::cell::UnsafeCell;


/// A container that can have multiple immutable or mutable references to the wrapped value.
/// 
/// # Broken Borrow Checker Rules
/// 
/// * [ ] Variables must be initialised before use.
/// * [ ] Values can not be moved more than once.
/// * [ ] Values can not be moved while borrowed.
/// * [x] Values can not be accessed while mutably borrowed.
/// * [x] Values can not be mutated while immutably borrowed.
/// 
/// # Generics
/// 
/// * `T` : The type of the wrapped value.
/// 
/// # Warning
/// 
/// * This structure is not thread safe in most cases.
/// * You are responsible for preventing data races and undefined behaviour.
/// * IN MOST CASES THIS SHOULD NOT BE USED DUE TO THE UNPREDICTABLE AND DANGEROUS NATURE OF THIS SYSTEM.
/// 
/// # Examples
/// 
/// ```
/// use pholib::MultiRef;
/// let multiref = unsafe {MultiRef::new(10)};
/// 
/// let a = multiref.get_ref();
/// let b = multiref.get_mut();
/// assert_eq!(*a, 10);
/// assert_eq!(*b, 10);
/// 
/// *b += 3;
/// assert_eq!(*a, 13);
/// assert_eq!(*b, 13);
/// ```
/// 
/// # Examples
/// 
/// ```
/// use pholib::MultiRef;
/// struct Test {
///     pub a : i32,
///     pub b : bool
/// }
/// let multiref = unsafe {MultiRef::new(
///     Test {
///         a : 1,
///         b : false
///     }
/// )};
/// 
/// let i = multiref.get_ref();
/// assert_eq!((*i).a, 1);
/// assert_eq!((*i).b, false);
/// 
/// let x = multiref.get_mut();
/// let y = multiref.get_mut();
/// let z = multiref.get_mut();
/// (*x).a += 10;
/// (*y).b = true;
/// (*z).a += 7;
/// assert_eq!((*i).a, 18);
/// assert_eq!((*i).b, true);
/// 
/// let unwrapped = multiref.unwrap();
/// assert_eq!(unwrapped.a, 18);
/// assert_eq!(unwrapped.b, true);
/// ```
/// 
pub struct MultiRef<T>(UnsafeCell<T>);

impl<'l, T> MultiRef<T> {

    /// Create a new `MultiRef` instance.
    /// Because of the unsafe nature of this structure, the `new` function must be wrapped in `unsafe`.
    /// 
    /// # Arguments
    /// 
    /// * `object` : The object to wrap in the created `MultiRef`.
    /// 
    /// # Returns
    /// 
    /// The created `MultiRef` instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// ```
    /// 
    pub unsafe fn new(object : T) -> MultiRef<T> {
        return MultiRef(object.into())
    }

    /// Get an immutable reference to the wrapped value.
    /// Can be used simultaneously with `get_mut()`s or other `get_ref()`s.
    ///
    /// # Returns
    /// 
    /// An immutable reference to the wrapped value.
    /// 
    /// # Examples
    ///
    /// Basic
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// let i = multiref.get_ref();
    /// assert_eq!(*i, 10);
    /// ```
    ///
    /// Multiple immutable references.
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// let a = multiref.get_ref();
    /// let b = multiref.get_ref();
    /// assert_eq!(*a, 10);
    /// assert_eq!(*b, 10);
    /// ```
    /// 
    pub fn get_ref(&self) -> &T {
        return unsafe {& *(&self.0).get()};
    }

    /// Get a mutable reference to the wrapped value.
    /// Can be used simultaneously with `get_ref()`s or other `get_mut()`s.
    ///
    /// # Returns
    /// 
    /// A mutable reference to the wrapped value.
    /// 
    /// # Examples
    ///
    /// Basic
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// let a = multiref.get_mut();
    /// let b = multiref.get_mut();
    /// assert_eq!(*a, 10);
    /// assert_eq!(*b, 10);
    /// 
    /// *a += 1;
    /// assert_eq!(*a, 11);
    /// assert_eq!(*b, 11);
    /// 
    /// *b += 2;
    /// assert_eq!(*a, 13);
    /// assert_eq!(*b, 13);
    /// ```
    ///
    /// Multiple references.
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// let i = multiref.get_ref();
    /// let a = multiref.get_mut();
    /// let b = multiref.get_mut();
    /// assert_eq!(*i, 10);
    /// assert_eq!(*a, 10);
    /// assert_eq!(*b, 10);
    /// 
    /// *a += 1;
    /// assert_eq!(*i, 11);
    /// assert_eq!(*a, 11);
    /// assert_eq!(*b, 11);
    /// 
    /// *b += 2;
    /// assert_eq!(*i, 13);
    /// assert_eq!(*a, 13);
    /// assert_eq!(*b, 13);
    /// ```
    ///
    pub fn get_mut(&self) -> &mut T {
        return unsafe {&mut *(&self.0).get()};
    }

    /// Return the wrapped value and drop the `MultiRef`.
    ///
    /// # Returns
    /// 
    /// The wrapped value.
    /// 
    /// # Examples
    ///
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// assert_eq!(multiref.unwrap(), 10);
    /// ```
    ///
    /// ```
    /// use pholib::MultiRef;
    /// let multiref = unsafe {MultiRef::new(10)};
    /// 
    /// let a = multiref.get_mut();
    /// let b = multiref.get_mut();
    /// *a += 1;
    /// *b += 2;
    /// assert_eq!(multiref.unwrap(), 13);
    /// 
    /// // `multiref` can no longer be used because `unwrap()` dropped it.
    /// ```
    ///
    pub fn unwrap(self) -> T {
        return self.0.into_inner();
    }

}





#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn multiref() {
        let multiref = unsafe {MultiRef::new(10)};

        let a = multiref.get_mut();
        let b = multiref.get_mut();
        *a += 1;
        *b += 2;
        assert_eq!(multiref.unwrap(), 13);
    }

    #[test]
    fn immut_and_mut() {
        let multiref = unsafe {MultiRef::new(10)};

        let i = multiref.get_ref();
        assert_eq!(*i, 10);

        let a = multiref.get_mut();
        let b = multiref.get_mut();
        *a += 1;
        *b += 2;
        assert_eq!(*i, 13);
        assert_eq!(multiref.unwrap(), 13);
    }

    struct Test {
        pub a : i32,
        pub b : bool
    }
    
    #[test]
    fn struct_mut() {
        let multiref = unsafe {MultiRef::new(
            Test {
                a : 1,
                b : false
            }
        )};

        let x = multiref.get_mut();
        let y = multiref.get_mut();
        let z = multiref.get_mut();
        (*x).a += 10;
        (*y).b = true;
        (*z).a += 7;
    
        let unwrapped = multiref.unwrap();
        assert_eq!(unwrapped.a, 18);
        assert_eq!(unwrapped.b, true);
    }
    
    #[test]
    fn struct_immut_and_mut() {
        let multiref = unsafe {MultiRef::new(
            Test {
                a : 1,
                b : false
            }
        )};

        let i = multiref.get_ref();
        assert_eq!((*i).a, 1);
        assert_eq!((*i).b, false);

        let x = multiref.get_mut();
        let y = multiref.get_mut();
        let z = multiref.get_mut();
        (*x).a += 10;
        (*y).b = true;
        (*z).a += 7;
        assert_eq!((*i).a, 18);
        assert_eq!((*i).b, true);

        let unwrapped = multiref.unwrap();
        assert_eq!(unwrapped.a, 18);
        assert_eq!(unwrapped.b, true);
    }

    // THIS IS, FOR THE MOST PART, A TERRIBLE IDEA. IF YOU DO THIS, MAKE SURE YOU KNOW WHAT YOU'RE DOING.
    #[test]
    fn threads() {
        let a = 10;
        let b = 10;
        let c = 100;
        let d = 1;

        let multiref = unsafe {MultiRef::new(a)};

        for _ in 0..b {
            thread::scope(|scope| {
                let mutref = multiref.get_mut();
                scope.spawn(|| {
                    for _ in 0..c {
                        *mutref += d;
                    }
                });
            });
        }

        assert_eq!(multiref.unwrap(), a + b * c * d);
    }

}
