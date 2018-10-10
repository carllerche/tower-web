extern crate tower_web;
extern crate bytes;
extern crate flate2;
extern crate futures;
extern crate rand;

use tower_web::util::buf_stream::BufStream;
use tower_web::util::buf_stream::deflate::CompressStream;

use bytes::Bytes;
use futures::{Poll, Future};
use flate2::Compression;
use flate2::read::DeflateDecoder;
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard};

use std::cmp;
use std::io;
use std::io::prelude::*;

macro_rules! assert_round_trip {
    ($stream:expr, $expect:expr) => {{
        let data: Vec<u8> = $stream.collect().wait().unwrap();
        let mut decoder = DeflateDecoder::new(&data[..]);
        let mut actual = vec![];
        decoder.read_to_end(&mut actual).unwrap();
        assert_eq!(&actual[..], &$expect[..]);
    }}
}

#[test]
fn single_chunk() {
    let data: Vec<u8> = Standard.sample_iter(&mut thread_rng())
        .take(16 * 1024)
        .collect();

    let deflate = CompressStream::new(
        Mock::single_chunk(data.clone()),
        Compression::fast());

    assert_round_trip!(deflate, data);
}

#[test]
fn multi_chunk() {
    let data: Vec<u8> = Standard.sample_iter(&mut thread_rng())
        .take(16 * 1024)
        .collect();

    let deflate = CompressStream::new(
        Mock::rand_chunks(data.clone()),
        Compression::fast());

    assert_round_trip!(deflate, data);
}

struct Mock {
    chunks: Vec<Bytes>,
}

impl Mock {
    pub fn single_chunk(data: Vec<u8>) -> Mock {
        Mock { chunks: vec![data.into()] }
    }

    pub fn rand_chunks(data: Vec<u8>) -> Mock {
        let mut data = Bytes::from(data);
        let max = data.len() / 4;

        let mut rng = thread_rng();
        let mut chunks = vec![];

        while !data.is_empty() {
            let n = cmp::min(rng.gen_range(1, max), data.len());
            chunks.push(data.split_to(n));
        }

        Mock { chunks }
    }
}

impl BufStream for Mock {
    type Item = io::Cursor<Bytes>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.chunks.is_empty() {
            return Ok(None.into());
        }

        let chunk = self.chunks.remove(0);
        let buf = io::Cursor::new(chunk);

        Ok(Some(buf).into())
    }
}
