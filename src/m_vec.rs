
use std::{mem::size_of, ptr::{null, self}, ffi::c_void};

use libc::{malloc, free};

struct MVec<T> {
    values: *mut T,
    count: usize,
    capacity: usize
}

impl<T> MVec<T> {
    pub fn new() -> Self {
        MVec { values: unsafe{malloc(0)} as *mut T, count: 0, capacity: 0 }
    }
    pub fn add(&mut self, element: T) {
        if self.count == self.capacity {
            let temp: *mut T = self.values;                      //remember the privios array
    unsafe{ self.values = malloc(self.capacity + size_of::<T>() * 16) as *mut T;  }//alocate with the new capacity
            self.capacity += 16;                                 //increse the capacity
            for i in 0..self.count {
                unsafe {
                    let ptr = self.values;
                    let dst = ptr.add(i);
                    let src = temp.add(i);
                    ptr::write(dst, ptr::read(src));
                } //copys the values from previos array


            }

            unsafe { free(temp as *mut c_void); } //dealocate previos array
        }

        //values[count] = element;
        unsafe { ptr::write(self.values.add(self.count), element); }

        self.count += 1;
    }
}