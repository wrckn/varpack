use std::{
    cell::{
        Ref,
        RefMut,
        RefCell
    },
    rc::Rc,
    ops::{
        Deref,
        DerefMut
    }
};

/// A wrapper type for Rc<RefCell<T>>
#[derive(Clone)]
pub struct Ptr<T> {
    inner_rc: Rc<RefCell<T>>
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
}