pub mod common {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::str::FromStr;
    use thiserror::Error;

    pub struct LineReader {
        f: BufReader<File>,
    }

    impl Iterator for LineReader {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            loop {
                let mut line = String::new();
                let n = self.f.read_line(&mut line).unwrap();
                // TODO: Figure out how to do this without making a copy.
                let line = line.trim();
                if line == "" {
                    if n == 0 {
                        return None;
                    }
                    continue;
                }
                return Some(line.to_owned());
            }
        }
    }

    #[derive(Error, Debug)]
    pub enum CommonError {
        #[error("unable to open file {path:?}")]
        FileNotFound { source: io::Error, path: String },

        #[error("unable to parse {line:?}")]
        ParseError {
            source: Box<dyn std::error::Error + Send + Sync>,
            line: String,
        },
    }

    // Skips blank lines.
    pub fn read_lines(path: &str) -> Result<LineReader, CommonError> {
        match File::open(path) {
            Ok(f) => Ok(LineReader {
                f: BufReader::new(f),
            }),
            Err(e) => Err(CommonError::FileNotFound {
                source: e,
                path: path.to_owned(),
            }),
        }
    }

    // Parses every item in the given iterator using FromStr.
    pub fn parse_all<'a, T, I, S>(items: I) -> Result<Vec<T>, CommonError>
    where
        T: FromStr,
        S: AsRef<str>,
        I: Iterator<Item = S>,
        T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut v = Vec::new();
        for part in items {
            let part = part.as_ref().trim();
            match part.parse::<T>() {
                Ok(n) => v.push(n),
                Err(e) => {
                    return Err(CommonError::ParseError {
                        line: part.to_owned(),
                        source: e.into(),
                    });
                }
            }
        }
        Ok(v)
    }

    // Helper methods for iterators.
    pub trait StrIterator: Iterator {
        // Parses every item in the given iterator using FromStr.
        fn parse_all<'a, S, T>(self) -> Result<Vec<T>, CommonError>
        where
            S: AsRef<str>,
            Self: Iterator<Item = S> + Sized,
            T: FromStr,
            // T::Err: 'static + std::error::Error + Send + Sync,
            T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            crate::common::parse_all(self)
        }
    }

    impl<T> StrIterator for T where T: Iterator {}
}
