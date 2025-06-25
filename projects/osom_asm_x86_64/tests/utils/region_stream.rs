#![cfg(target_arch = "x86_64")]
#![allow(dead_code)]

use std::io::Write;

pub struct RegionStream {
    allocated_space: region::Allocation,
    length: usize,
}

impl RegionStream {
    #[inline(always)]
    fn page_size() -> usize {
        core::cmp::max(region::page::size(), 4096)
    }

    pub fn new() -> Self {
        let prot = region::Protection::READ_WRITE_EXECUTE;
        let base = region::alloc(Self::page_size(), prot).unwrap();
        Self {
            allocated_space: base,
            length: 0,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &as_slice(&self.allocated_space)[0..self.length]
    }

    #[cfg(target_arch = "x86_64")]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const u8 {
        self.allocated_space.as_ptr()
    }
}

impl Write for RegionStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        let new_size = self.length + buf_len;
        let capacity = self.allocated_space.len();
        if new_size > capacity {
            let page_size = Self::page_size();
            let new_capacity = ((2 * new_size / page_size) + 1) * page_size;
            let mut new_alloc = region::alloc(new_capacity, region::Protection::READ_WRITE_EXECUTE).unwrap();
            let src = &as_slice(&self.allocated_space)[0..self.length];
            let dst = &mut as_mut_slice(&mut new_alloc)[0..self.length];
            dst.copy_from_slice(src);
            self.allocated_space = new_alloc;
        }

        let dst = &mut as_mut_slice(&mut self.allocated_space)[self.length..self.length + buf_len];
        dst.copy_from_slice(buf);
        self.length += buf_len;
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[inline(always)]
fn as_slice(alloc: &region::Allocation) -> &[u8] {
    unsafe { std::slice::from_raw_parts(alloc.as_ptr(), alloc.len()) }
}

#[inline(always)]
fn as_mut_slice(alloc: &mut region::Allocation) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(alloc.as_mut_ptr(), alloc.len()) }
}
