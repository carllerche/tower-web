use super::SizeHint;

use bytes::{Buf, BufMut};

pub trait FromBufStream {
    fn with_capacity(hint: &SizeHint) -> Self;

    fn extend<T: Buf>(&mut self, buf: &mut T);
}

impl FromBufStream for Vec<u8> {
    fn with_capacity(hint: &SizeHint) -> Vec<u8> {
        Vec::with_capacity(hint.lower())
    }

    fn extend<T: Buf>(&mut self, buf: &mut T) {
        self.put(buf);
    }
}
