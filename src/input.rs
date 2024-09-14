use std::io::{Read, BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;

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
}
