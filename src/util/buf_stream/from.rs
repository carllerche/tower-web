use super::SizeHint;

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait FromBufStream {
    type Builder;

    fn builder(hint: &SizeHint) -> Self::Builder;

    fn extend<T: Buf>(builder: &mut Self::Builder, buf: &mut T);

    fn build(builder: Self::Builder) -> Self;
}

impl FromBufStream for Vec<u8> {
    type Builder = Vec<u8>;

    fn builder(hint: &SizeHint) -> Vec<u8> {
        Vec::with_capacity(hint.lower())
    }

    fn extend<T: Buf>(builder: &mut Self, buf: &mut T) {
        builder.put(buf);
    }

    fn build(builder: Self) -> Self {
        builder
    }
}

impl FromBufStream for Bytes {
    type Builder = BytesMut;

    fn builder(hint: &SizeHint) -> BytesMut {
        BytesMut::with_capacity(hint.lower())
    }

    fn extend<T: Buf>(builder: &mut Self::Builder, buf: &mut T) {
        builder.put(buf);
    }

    fn build(builder: Self::Builder) -> Self {
        builder.freeze()
    }
}
