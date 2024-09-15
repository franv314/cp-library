use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::iter;
use std::str::FromStr;
use std::fmt::{Debug, Display};

/// Buffered token-based input reader.
/// 
/// Provides a token-by-token input reader over any [`Read`] type,
/// with automatic parsing into any [`FromStr`] type.
pub struct InputReader<R: Read> {
    reader: BufReader<R>,
    tokens: Vec<String>,
}

impl<R: Read> InputReader<R> {

    /// Builds an input reader over a given reader, consuming it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::io;
    /// use cp_library::inout::InputReader;
    /// 
    /// let mut reader = InputReader::new(io::stdin());
    /// ```
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

    /// Extracts a single token and parses into a [`FromStr`] type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cp_library::inout::InputReader;
    /// 
    /// let mut reader = InputReader::new("123 abc".as_bytes());
    /// assert_eq!(reader.get::<i32>(), 123);
    /// assert_eq!(reader.get::<String>(), String::from("abc"));
    /// ```
    ///
    /// # Panics
    /// 
    /// If the next token fails to be parsed into `T`.
    /// 
    /// ```should_panic
    /// use cp_library::inout::InputReader;
    /// 
    /// let mut reader = InputReader::new("abc".as_bytes());
    /// let x: i32 = reader.get();
    /// ```
    pub fn get<T: FromStr>(&mut self) -> T
        where <T as FromStr>::Err: Debug
    {
        self.get_token().as_str().parse().expect("Invalid token for this type!")
    }

    /// Extracts `size` tokens of the same [`FromStr`] type
    /// and collect them in a [`Vec`].
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::inout::InputReader;
    ///
    /// let mut reader = InputReader::new("123 456 789".as_bytes());
    /// assert_eq!(reader.get_vec::<i32>(3), vec![123, 456, 789]);
    /// ```
    ///
    /// # Panics
    ///
    /// If any of the next `size` token fail to be parsed into `T`
    /// ```should_panic
    /// use cp_library::inout::InputReader;
    ///
    /// let mut reader = InputReader::new("123 456 abc".as_bytes());
    /// let x = reader.get_vec::<i32>(3);
    /// ```
    pub fn get_vec<T: FromStr>(&mut self, size: usize) -> Vec<T>
        where <T as FromStr>::Err: Debug
    {
        iter::from_fn(|| Some(self.get::<T>())).take(size).collect::<Vec<_>>()
    }
}

/// Buffered output writer.
///
/// Provides a buffered writer over any [`Write`] type,
/// able to write any [`Display`] type.
pub struct OutputWriter<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> OutputWriter<W> {

    /// Builds an output writer over a given writer, consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use std::process::Output;
    /// use cp_library::inout::OutputWriter;
    ///
    /// let mut reader = OutputWriter::new(io::stdout());
    /// ```
    pub fn new(writer: W) -> Self {
        Self { writer: BufWriter::new(writer) }
    }

    /// Writes `val` followed by a delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::inout::OutputWriter;
    ///
    /// let mut buf = Vec::new();
    /// {
    ///     let mut writer = OutputWriter::new(&mut buf);
    ///
    ///     writer.put_d(&123, ' ');
    ///     writer.put_d(&"abc", ' ');
    /// }
    ///
    /// assert_eq!(buf, b"123 abc ".to_vec())
    /// ```
    pub fn put_d<T: Display>(&mut self, val: &T, delim: char) {
        write!(self.writer, "{val}{delim}").expect("Could not print!")
    }

    /// Writes `val`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cp_library::inout::OutputWriter;
    ///
    /// let mut buf = Vec::new();
    /// {
    ///     let mut writer = OutputWriter::new(&mut buf);
    ///
    ///     writer.put(&"abc");
    ///     writer.put(&"xyz");
    /// }
    ///
    /// assert_eq!(buf, b"abcxyz".to_vec())
    /// ```
    pub fn put<T: Display>(&mut self, val: &T) {
        write!(self.writer, "{val}").expect("Could not print!")
    }
}
