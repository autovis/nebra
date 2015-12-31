
use std::fmt;
use std::default::Default;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
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
    
    // Formula
    let mut istream : Stream<f32> = Stream::new("istream", 10);
    let mut dsize = Doublesize::new(&istream);
    istream.next();
    istream.set(1.0, 0);
    dsize.update();
    println!("{:?}", istream);
    
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
    
    pub fn index(&mut self) -> isize {
        self.index
    }
}

impl<T:fmt::Debug> fmt::Debug for Stream<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.buffer)
    }
}

/////////////////////////////////////////////////////////////////////

// Formula (general)
pub trait Formula<I, O> {
    fn init(&mut self) -> Result<(), &'static str>;
    fn update(&mut self) -> Result<(), &'static str>;
    fn index(&mut self) -> isize;
}

/////////////////////////////////////////////////////////////////////

// Formula (specific)
#[allow(dead_code)]
pub struct Doublesize<'a> {
    input: &'a Stream<f32>,
    output: Stream<f32>
}

impl<'a> Doublesize<'a> {
    pub fn new(input : &'a Stream<f32>) -> Doublesize {
        Doublesize {
            input: input,
            output: Stream::new("output", 100)
        }
    }
}

impl<'a> Formula<&'a Stream<f32>, Stream<f32>> for Doublesize<'a> {
    fn init(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
    fn update(&mut self) -> Result<(), &'static str> {
        if self.index() == -1 {return Err("Uninitialized output stream")}
        let out = match self.index() {
            0 => self.input.get(0) * 2.0,
            _ => Default::default()
        };
        self.output.next();
        self.output.set(out, 0);
        Ok(())
    }
    fn index(&mut self) -> isize {
        self.output.index()
    }
}
