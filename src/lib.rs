#![crate_name="bytes"]

/// A trait for objects that provide random and sequential access to bytes.
pub trait Buf {

    fn remaining(&self) -> usize;

    fn bytes(&self) -> &[u8];

    fn advance(&mut self, cnt: usize);

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
}

pub trait MutBuf: Buf {
    fn mut_bytes(&mut self) -> &mut [u8];
}