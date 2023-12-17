pub mod common {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::ops::{Index, IndexMut};
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

        #[error("mismatched rows: got {got:?}; expected {expected:?}")]
        MismatchedRowsError { got: usize, expected: usize },

        #[error(transparent)]
        InnerError(#[from] Box<dyn std::error::Error + Send + Sync>),
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

    pub struct Array2D<T> {
        data: Vec<T>,
        rows: usize,
        cols: usize,
    }

    impl<T> Array2D<T> {
        fn get_index(&self, row: usize, col: usize) -> Option<usize> {
            if row >= self.rows || col >= self.cols {
                None
            } else {
                Some(row * self.cols + col)
            }
        }

        pub fn get(&self, row: usize, col: usize) -> Option<&T> {
            self.get_index(row, col)
                .map(|i| self.data.get(i).expect("checked bounds"))
        }

        pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
            self.get_index(row, col)
                .map(|i| self.data.get_mut(i).expect("checked bounds"))
        }

        pub fn from_rows<I: IntoIterator<Item = Vec<T>>>(it: I) -> Result<Self, CommonError> {
            let mut data = Vec::new();
            let mut cols = None;

            for row in it {
                match cols {
                    Some(n) => {
                        if row.len() != n {
                            return Err(CommonError::MismatchedRowsError {
                                got: n,
                                expected: row.len(),
                            });
                        }
                    }
                    None => cols = Some(row.len()),
                }
                for r in row {
                    data.push(r);
                }
            }

            let cols = cols.unwrap_or(0usize);
            let rows = data.len();
            Ok(Array2D { data, rows, cols })
        }
    }

    impl<T> Index<(usize, usize)> for Array2D<T> {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> &T {
            self.get(index.0, index.1)
                .unwrap_or_else(|| panic!("index out of bounds: ({}, {})", index.0, index.1))
        }
    }

    impl<T> IndexMut<(usize, usize)> for Array2D<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            self.get_mut(index.0, index.1)
                .unwrap_or_else(|| panic!("index out of bounds: ({}, {})", index.0, index.1))
        }
    }

    impl<T> TryFrom<Vec<Vec<T>>> for Array2D<T> {
        type Error = CommonError;

        fn try_from(value: Vec<Vec<T>>) -> Result<Self, CommonError> {
            Array2D::from_rows(value)
        }
    }

    pub fn read_grid<T, E>(
        path: &str,
        f: fn(char) -> Result<T, E>,
    ) -> Result<Array2D<T>, CommonError>
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut v = Vec::new();
        for line in read_lines(path)? {
            let mut row = Vec::new();
            for c in line.chars() {
                match f(c) {
                    Ok(item) => row.push(item),
                    Err(e) => {
                        return Err(CommonError::InnerError(e.into()));
                    }
                }
            }
            v.push(row);
        }
        Ok(Array2D::from_rows(v)?)
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
            T::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
            crate::common::parse_all(self)
        }
    }

    impl<T> StrIterator for T where T: Iterator {}
}
