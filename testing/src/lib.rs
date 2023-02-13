//! A cycled buffer that rewrites its content when there is no more space in allocated vector.
//!
//! ```
//! use std::io::Write;
//! use std::ops::Deref;
//! use testing::CycledBuffer;
//! let mut buf = CycledBuffer::new(4);
//! let _ = buf.write(&[1, 2]);
//! assert_eq!(buf.deref(), &vec![1, 2, 0, 0]);
//! let _ = buf.write(&[3, 4, 5, 6]);
//! assert_eq!(buf.deref(), &vec![5, 6, 3, 4]);
//! ```
#![feature(stmt_expr_attributes)]
use std::io::Write;
use std::ops::Deref;

pub struct CycledBuffer {
    buf: Vec<u8>,
    ptr: usize,
    #[cfg(test)]
    total_bytes: usize,
}

impl CycledBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buf: vec![0; size],
            ptr: 0,
            #[cfg(test)]
            total_bytes: 0,
        }
    }
}

impl Deref for CycledBuffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl Write for CycledBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf.iter() {
            self.buf[self.ptr] = *byte;
            self.ptr = (self.ptr + 1) % self.buf.len();
        }
        let n = buf.len();
        #[cfg(test)]
        {
            self.total_bytes += n;
        }
        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_tracks_bytes() {
        let mut buf = CycledBuffer::new(10);
        let _ = buf.write(&[0; 16]);
        let _ = buf.write(&[0; 16]);
        assert_eq!(buf.total_bytes, 32);
    }
}
