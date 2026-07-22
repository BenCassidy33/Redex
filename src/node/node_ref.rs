use std::{cell::{Ref, RefCell, RefMut}, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct NodeRef<T> {
    pub(crate) inner: Rc<RefCell<T>>,
}

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for NodeRef<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> NodeRef<T> {
    pub fn new(item: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(item)),
        }
    }

    #[inline]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

