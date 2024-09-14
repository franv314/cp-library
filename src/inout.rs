use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::iter;
use std::str::FromStr;
use std::fmt::{Debug, Display};

pub struct InputReader<R: Read> {
    reader: BufReader<R>,
    tokens: Vec<String>,
}

impl<R: Read> InputReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            tokens: vec![]
        }
    }

    fn get_token(&mut self) -> String {
        loop {
            if let Some(token) = self.tokens.pop() {
                return token;
            }

            let mut line = String::new();
            self.reader.read_line(&mut line).expect("Could not read!");
            self.tokens = line.split_whitespace().map(String::from).rev().collect();
        }
    }

    pub fn get<T: FromStr>(&mut self) -> T
        where <T as FromStr>::Err: Debug
    {
        self.get_token().as_str().parse().expect("Invalid token for this type!")
    }

    pub fn get_vec<T: FromStr>(&mut self, size: usize) -> Vec<T>
        where <T as FromStr>::Err: Debug
    {
        iter::from_fn(|| Some(self.get::<T>())).take(size).collect::<Vec<_>>()
    }
}

pub struct OutputWriter<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> OutputWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer: BufWriter::new(writer) }
    }

    pub fn put_d<T: Display>(&mut self, val: &T, delim: char) {
        write!(self.writer, "{val}{delim}").expect("Could not print!")
    }
    
    pub fn put<T: Display>(&mut self, val: &T) {
        write!(self.writer, "{val}").expect("Could not print!")
    }
}
