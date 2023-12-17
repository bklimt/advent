pub mod common {
    use anyhow::{Context, Result};
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

    // Parses every item in the given iterator using FromStr.
    pub fn parse_all<'a, T, I>(items: I) -> Result<Vec<T>, anyhow::Error>
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
        fn parse_all<'a, T>(self) -> Result<Vec<T>, anyhow::Error>
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
