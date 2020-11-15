#[cfg(any(unix))]
use libc::{
    mmap,
    munmap,
    PROT_EXEC,
    PROT_READ,
    PROT_WRITE,
    MAP_ANONYMOUS,
    MAP_PRIVATE,
};

use std::{ptr, io, slice};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asm {
    bytes: Vec<u8>,
}

impl Asm {

    pub fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(64)
        }
    }

    pub fn put(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn inside(&self) -> &[u8] {&self.bytes}

    #[cfg(any(unix))]
    pub unsafe fn prepare<T>(&self) -> Result<Compiled<T>, io::Error> {
        let ptr = mmap(
            ptr::null_mut(),
            self.bytes.len(),
            PROT_EXEC | PROT_READ | PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0
        );
        if ptr == ptr::null_mut() {
            Err(io::Error::last_os_error())
        } else {
            ptr::copy(self.bytes.as_ptr(), ptr as *mut u8, self.bytes.len());
            Ok(Compiled {
                func: ptr as *mut T,
                size: self.bytes.len()
            })
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct Compiled<T> {
    func: *mut T,
    size: usize,
}

impl<T> Compiled<T> {
    pub unsafe fn func(&self) -> T
        where
            T: Copy
    {
        *(&self.func as *const *mut T as *const T)
    }

    pub fn len(&self) -> usize {self.size}

    pub fn bytecode(&self) -> Vec<u8> {
        Vec::from(unsafe {
            slice::from_raw_parts(self.func as *const u8, self.size)
        })
    }

}

#[cfg(any(unix))]
impl<T> Drop for Compiled<T> {

    fn drop(&mut self) {
        unsafe {
            let result = munmap(self.func as *mut _, self.size);
            debug_assert!(result >= 0);
        }
    }

}