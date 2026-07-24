use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    ops::Deref,
    rc::Rc,
};

use crate::ast::{Node, NodeVariant, deep_clone::NodeDeepClone, variable::Variable};

#[derive(Debug, PartialEq)]
pub struct NodeRef<T> {
    pub(crate) inner: Rc<RefCell<T>>,
}

impl<T: PartialEq> PartialEq<T> for NodeRef<T> {
    fn eq(&self, other: &T) -> bool {
        *self.borrow() == *other
    }
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

impl NodeRef<Node> {
    #[inline]
    pub fn deep_clone(&self) -> NodeDeepClone {
        Node::deep_clone(self)
    }
}

impl<T: Display> Display for NodeRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.borrow())
    }
}

impl NodeRef<Node> {
    #[inline]
    pub fn substitute(&self, var: &NodeRef<Variable>, replacement: NodeRef<Node>) {
        Node::substitute(self, var, replacement);
    }

    #[inline]
    pub fn variant(&self) -> NodeVariant {
        self.into()
    }
}
