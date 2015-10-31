
use std::fmt;
use std::default::Default;

#[allow(dead_code)]
fn main() {

    // int stream
    let mut stream1 : Stream<isize> = Stream::new("int_stream", 10);
    stream1.next();
    stream1.set(1, 0);
    stream1.next();
    stream1.set(2, 0);
    stream1.next();
    stream1.set(3, 0);
    stream1.next();
    stream1.set(4, 0);
    //stream1.next();
    
    println!("{:?}", stream1);
    println!("{:?} {:?} {:?}", stream1.get(0), stream1.get(1), stream1.get(2));
    println!("idx: {:?}", stream1.index);

    // str stream
    let mut stream2 : Stream<&'static str> = Stream::new("str_stream", 3);
    stream2.next();
    stream2.set("abc", 0);
    stream2.next();
    stream2.set("def", 0);
    stream2.next();
    stream2.set("ghi", 0);
    stream2.next();
    println!("{:?}", stream2);

    // Option<int> stream
    let mut stream3 : Stream<Option<i32>> = Stream::new("optint_stream", 3);
    stream3.next();
    stream3.set(Some(10), 0);
    stream3.next();
    println!("{:?}", stream3);
}

// ======================================================================

#[allow(dead_code)]
pub struct Stream<T> {
	id: &'static str,
    size: usize,
	index: isize,
	buffer: Vec<T>,
}

#[allow(dead_code)]
impl<T:Default> Stream<T> {

    pub fn new(id: &'static str, size: usize) -> Stream<T> {
        Stream {
            size: size,
            buffer: Vec::with_capacity(size),
            index: -1,
            id: id
        }
    }

	pub fn get(&self, bars_ago: usize) -> &T {
        let size = self.buffer.capacity();
        &self.buffer[((self.index as usize) - bars_ago) % size]
	}

    pub fn set(&mut self, value: T, bars_ago: usize) {
        let idx = (self.index as usize) - bars_ago;
        let size = self.buffer.capacity();
        self.buffer[idx % size] = value;
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.buffer.len() < self.buffer.capacity() {
            self.buffer.push(Default::default());
        } else {
            let size = self.buffer.capacity();
            self.buffer[(self.index as usize) % size] = Default::default();
        }
    }
}

impl<T:fmt::Debug> fmt::Debug for Stream<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.buffer)
    }
}

/////////////////////////////////////////////////////////////////////

// Input
pub trait Input {}
impl Input for () {}
impl<A> Input for (Stream<A>) {}
impl<A, B> Input for (Stream<A>, Stream<B>) {}
impl<A, B, C> Input for (Stream<A>, Stream<B>, Stream<C>) {}

// Output
pub trait Output {}

// Formula (general)
pub trait Formula<I, O> where I : Input, O : Output {
    fn init() -> Result<(), &'static str>;
    fn update() -> Result<(), &'static str>;
}

/////////////////////////////////////////////////////////////////////

// Formula (specific)
#[allow(dead_code)]
pub struct EMA<I, O> where I : Input, O : Output{
    input: I,
    output: O
}

impl<I, O> Formula<I, O> for EMA<I, O> where I : Input, O : Output {
    fn init() -> Result<(), &'static str> {
        Ok(())
    }
    fn update() -> Result<(), &'static str> {
        Ok(())
    }
}
