use std::{
    cell::{
        Ref,
        RefMut,
        RefCell
    },
    rc::Rc,
    ops::{
        Deref,
        DerefMut,
        
    },
    marker::PhantomData
};

/// A wrapper type for Rc<RefCell<T>>
pub struct Ptr<T> {
    inner_rc: Rc<RefCell<T>>,
}

impl<T> Ptr<T> {
    /// Creates a new Rc<RefCell<T>> wrapper
    pub fn new(item: T) -> Self {
        Self {
            inner_rc: Rc::new(RefCell::new(item))
        }
    }

    /// Get an immutable reference to the content
    pub fn get<'r>(&'r self) -> Ref<'r, T> {
        self.inner_rc.as_ref().borrow()
    }
    
    /// Get a mutable reference to the content
    pub fn get_mut<'r>(&'r self) -> RefMut<'r, T> {
        self.inner_rc.as_ref().borrow_mut()
    }

    /// Clones a pointer
    pub fn clone_ptr(&self) -> Ptr<T> {
        let rc = self.inner_rc.clone();
        Self {
            inner_rc: rc
        }
    }
}