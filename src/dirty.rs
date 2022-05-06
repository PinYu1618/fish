//! Code Ref: https://github.com/rcore-os/rcore-fs/rcore-fs/src/dirty.rs

use core::ops::{Deref, DerefMut};
use core::fmt;

pub struct Dirty<T> {
    value: T,
    dirty: bool,
}

impl<T> Dirty<T> {
    pub fn new(val: T) -> Dirty<T> {
        Dirty { value: val, dirty: false }
    }

    pub fn new_dirty(val: T) -> Dirty<T> {
        Dirty { value: val, dirty: true }
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn sync(&mut self) {
        self.dirty = false;
    }
}

impl<T> Deref for Dirty<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Dirty<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.value
    }
}

impl<T> Drop for Dirty<T> {
    fn drop(&mut self) {
        assert!(!self.dirty, "data dirty when dropping");
    }
}

impl<T: fmt::Debug> fmt::Debug for Dirty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = if self.dirty { "Dirty" } else { "Clean" };
        write!(f, "[{}] {:?}", tag, self.value)
    }
}
