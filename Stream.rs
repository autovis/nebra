
use std::fmt;
use std::default::Default;

#[allow(dead_code)]
fn main() {

    // int stream
    let mut stream1 : Stream<i32> = Stream::new("int_stream", 1i32);
    stream1.next();
    stream1.set(7, 0);
    stream1.next();
    println!("{:?}", stream1);

    // str stream
    let mut stream2 : Stream<&'static str> = Stream::new("str_stream", 2i32);
    stream2.next();
    stream2.set("asdf", 0);
    stream2.next();
    println!("{:?}", stream2);

    // Option<int> stream
    /*
    let mut stream3 : Stream<Option<i32>> = Stream::new("optint_stream", 3i32);
    stream3.next();
    stream3.set(Some(10i32), 0);
    stream3.next();
    println!("{:?}", stream3);
    */
}

// ======================================================================

#[allow(dead_code)]
pub struct Stream<T> {

    params: i32,
	id: &'static str,

	index: usize,
	buffer: Vec<T>,
}

#[allow(dead_code)]
impl<T:Default> Stream<T> {

    pub fn new(id: &'static str, params: i32) -> Stream<T> {

        Stream {
            params: params,
            buffer: Vec::with_capacity(100),
            index: -1,
            id: id
        }

    }

	pub fn get(&self, bars_ago: usize) -> &T {

        // sanity checks here
        let size = self.buffer.capacity();
        &self.buffer[(self.index - bars_ago) % size]
	}

    pub fn set(&mut self, value: T, bars_ago: usize) {
        let idx = self.index - bars_ago;
        let size = self.buffer.capacity();
        self.buffer[idx % size] = value;
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.buffer.len() < self.buffer.capacity() {
            self.buffer.push(Default::default());
        } else {
            let size = self.buffer.capacity();
            self.buffer[self.index % size] = Default::default();
        }
    }
}

impl<T:fmt::Debug> fmt::Debug for Stream<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(format!("{:?}", self.buffer))
    }
}
