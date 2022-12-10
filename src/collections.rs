use crate::hash::DJB2;
use alloc::alloc::{alloc, dealloc, realloc};
use core::alloc::Layout;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        let cap = if core::mem::size_of::<T>() == 0 {
            !0
        } else {
            0
        };

        Self {
            ptr: 0 as *mut T,
            len: 0,
            cap,
        }
    }

    unsafe fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        let new_ptr = if self.cap == 0 {
            alloc(new_layout)
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr as *mut u8;
            realloc(old_ptr, old_layout, new_layout.size())
        };

        self.ptr = new_ptr as *mut T;
        self.cap = new_cap;
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            unsafe { self.grow() };
        }

        unsafe {
            core::ptr::write(self.ptr.add(self.len), value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(core::ptr::read(self.ptr.add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: `<=` because it's valid to insert after everything
        // which would be equivalent to push.
        assert!(index <= self.len, "index out of bounds");

        unsafe {
            if self.cap == self.len {
                self.grow();
            }
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            core::ptr::copy(
                self.ptr.add(index),
                self.ptr.add(index + 1),
                self.len - index,
            );
            core::ptr::write(self.ptr.add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        // Note: `<` because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = core::ptr::read(self.ptr.add(index));
            core::ptr::copy(
                self.ptr.add(index + 1),
                self.ptr.add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        let elem_size = core::mem::size_of::<T>();

        if self.cap != 0 && elem_size != 0 {
            unsafe {
                dealloc(self.ptr as *mut u8, Layout::array::<T>(self.cap).unwrap());
            }
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

pub struct Map<K: Hash, V> {
    keys: Vec<usize>,
    values: Vec<V>,
    _k: PhantomData<K>,
}

impl<K: Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            _k: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut hasher = DJB2::new();
        key.hash(&mut hasher);
        let k = hasher.finish() as usize;
        self.keys.push(k);
        self.values.push(value);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut hasher = DJB2::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;

        for k in 0..self.keys.len() {
            if self.keys[k] == hash {
                return self.values.get(k);
            }
        }

        None
    }
}
