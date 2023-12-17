pub mod common {
    use anyhow::{anyhow, Context, Result};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;

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

    // Skips blank lines.
    pub fn read_lines(path: &str) -> Result<LineReader> {
        let file = File::open(path).with_context(|| format!("unable to open file {:?}", path))?;
        let f = BufReader::new(file);
        Ok(LineReader { f })
    }

    pub struct Array2D<T> {
        data: Vec<T>,
        rows: usize,
        cols: usize,
    }

    impl<T> Array2D<T> {
        pub fn from_rows<I: IntoIterator<Item = Vec<T>>>(it: I) -> Result<Self> {
            let mut data = Vec::new();
            let mut cols = None;

            for row in it {
                match cols {
                    Some(n) => {
                        if row.len() != n {
                            return Err(anyhow!(
                                "row has length {}; previous row had length: {}",
                                row.len(),
                                n
                            ));
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

    impl<T> TryFrom<Vec<Vec<T>>> for Array2D<T> {
        type Error = anyhow::Error;

        fn try_from(value: Vec<Vec<T>>) -> Result<Self> {
            Array2D::from_rows(value)
        }
    }

    pub fn read_grid<T, E>(path: &str, f: fn(char) -> Result<T, E>) -> Result<Array2D<T>>
    where
        E: 'static + std::error::Error + Send + Sync,
    {
        let mut v = Vec::new();
        for line in read_lines(path)? {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(f(c)?);
            }
            v.push(row);
        }
        Ok(Array2D::from_rows(v)?)
    }

    // Parses every item in the given iterator using FromStr.
    pub fn parse_all<'a, T, I>(items: I) -> Result<Vec<T>>
    where
        T: FromStr,
        T::Err: 'static + std::error::Error + Send + Sync,
        I: Iterator<Item = &'a str>,
    {
        let mut v = Vec::new();
        for part in items {
            let part = part.trim();
            v.push(part.parse().context(format!("invalid item: {}", part))?);
        }
        Ok(v)
    }

    // Helper methods for iterators.
    pub trait StrIterator: Iterator {
        // Parses every item in the given iterator using FromStr.
        fn parse_all<'a, T>(self) -> Result<Vec<T>>
        where
            Self: Iterator<Item = &'a str> + Sized,
            T: FromStr,
            T::Err: 'static + std::error::Error + Send + Sync,
        {
            crate::common::parse_all(self)
        }
    }

    impl<T> StrIterator for T where T: Iterator {}
}
