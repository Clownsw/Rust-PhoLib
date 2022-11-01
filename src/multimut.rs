use std::cell::UnsafeCell;


/// A container that can have multiple immutable or mutable references to the wrapped value.
/// 
/// # Generics
/// 
/// * `T` : The type of the wrapped value.
/// 
/// # Warning
/// 
/// * This structure is absolutely not thread safe.
/// * You are responsible for preventing data races.
/// 
/// # Examples
/// 
/// ```
/// use pholib::MultiMut;
/// let multimut = unsafe {MultiMut::new(10)};
/// 
/// let a = multimut.get_ref();
/// let b = multimut.get_mut();
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
/// use pholib::MultiMut;
/// struct Test {
///     pub a : i32,
///     pub b : bool
/// }
/// let multimut = unsafe {MultiMut::new(
///     Test {
///         a : 1,
///         b : false
///     }
/// )};
/// 
/// let i = multimut.get_ref();
/// assert_eq!((*i).a, 1);
/// assert_eq!((*i).b, false);
/// 
/// let x = multimut.get_mut();
/// let y = multimut.get_mut();
/// let z = multimut.get_mut();
/// (*x).a += 10;
/// (*y).b = true;
/// (*z).a += 7;
/// assert_eq!((*i).a, 18);
/// assert_eq!((*i).b, true);
/// 
/// let unwrapped = multimut.unwrap();
/// assert_eq!(unwrapped.a, 18);
/// assert_eq!(unwrapped.b, true);
/// ```
/// 
pub struct MultiMut<T>(UnsafeCell<T>);

impl<'l, T> MultiMut<T> {

    /// Create a new `MultiMut` instance.
    /// Because of the unsafe nature of this structure, the `new` function must be wrapped in `unsafe`.
    /// 
    /// # Arguments
    /// 
    /// * `object` : The object to wrap in the created `MultiMut`.
    /// 
    /// # Returns
    /// 
    /// The created `MultiMut` instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// ```
    /// 
    pub unsafe fn new(object : T) -> MultiMut<T> {
        return MultiMut(object.into())
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
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// let i = multimut.get_ref();
    /// assert_eq!(*i, 10);
    /// ```
    ///
    /// Multiple immutable references.
    /// ```
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// let a = multimut.get_ref();
    /// let b = multimut.get_ref();
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
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// let a = multimut.get_mut();
    /// let b = multimut.get_mut();
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
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// let i = multimut.get_ref();
    /// let a = multimut.get_mut();
    /// let b = multimut.get_mut();
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

    /// Return the wrapped value and drop the `MultiMut`.
    ///
    /// # Returns
    /// 
    /// The wrapped value.
    /// 
    /// # Examples
    ///
    /// ```
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// assert_eq!(multimut.unwrap(), 10);
    /// ```
    ///
    /// ```
    /// use pholib::MultiMut;
    /// let multimut = unsafe {MultiMut::new(10)};
    /// 
    /// let a = multimut.get_mut();
    /// let b = multimut.get_mut();
    /// *a += 1;
    /// *b += 2;
    /// assert_eq!(multimut.unwrap(), 13);
    /// 
    /// // `multimut` can no longer be used because `unwrap()` dropped it.
    /// ```
    ///
    pub fn unwrap(self) -> T {
        return self.0.into_inner();
    }

}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn multimut() {
        let multimut = unsafe {MultiMut::new(10)};

        let a = multimut.get_mut();
        let b = multimut.get_mut();
        *a += 1;
        *b += 2;
        assert_eq!(multimut.unwrap(), 13);
    }

    #[test]
    fn immut_and_mut() {
        let multimut = unsafe {MultiMut::new(10)};

        let i = multimut.get_ref();
        assert_eq!(*i, 10);

        let a = multimut.get_mut();
        let b = multimut.get_mut();
        *a += 1;
        *b += 2;
        assert_eq!(*i, 13);
        assert_eq!(multimut.unwrap(), 13);
    }

    struct Test {
        pub a : i32,
        pub b : bool
    }
    
    #[test]
    fn struct_mut() {
        let multimut = unsafe {MultiMut::new(
            Test {
                a : 1,
                b : false
            }
        )};

        let x = multimut.get_mut();
        let y = multimut.get_mut();
        let z = multimut.get_mut();
        (*x).a += 10;
        (*y).b = true;
        (*z).a += 7;
    
        let unwrapped = multimut.unwrap();
        assert_eq!(unwrapped.a, 18);
        assert_eq!(unwrapped.b, true);
    }
    
    #[test]
    fn struct_immut_and_mut() {
        let multimut = unsafe {MultiMut::new(
            Test {
                a : 1,
                b : false
            }
        )};

        let i = multimut.get_ref();
        assert_eq!((*i).a, 1);
        assert_eq!((*i).b, false);

        let x = multimut.get_mut();
        let y = multimut.get_mut();
        let z = multimut.get_mut();
        (*x).a += 10;
        (*y).b = true;
        (*z).a += 7;
        assert_eq!((*i).a, 18);
        assert_eq!((*i).b, true);

        let unwrapped = multimut.unwrap();
        assert_eq!(unwrapped.a, 18);
        assert_eq!(unwrapped.b, true);
    }

}
