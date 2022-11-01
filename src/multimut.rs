use std::cell::UnsafeCell;

pub struct MultiMut<T>(UnsafeCell<T>);
impl<'l, T> MultiMut<T> {
    pub unsafe fn new(object : T) -> MultiMut<T> {
        return MultiMut(object.into())
    }
    /// Get an immutable reference to the stored value.
    /// Can be used simultaneously with `get_mut()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let multimut = unsafe {MultiMut::new(10)};
    /// let i = multimut.get_ref();
    /// assert_eq!(*i, 10);
    /// let a = multimut.get_mut();
    /// let b = multimut.get_mut();
    /// *a += 1;
    /// *b += 2;
    /// 
    /// assert_eq!(*i, 13);
    /// assert_eq!(multimut.unwrap(), 13);
    /// ```
    ///
    pub fn get_ref(&self) -> &T {
        return unsafe {& *(&self.0).get()};
    }
    pub fn get_mut(&self) -> &mut T {
        return unsafe {&mut *(&self.0).get()};
    }
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
