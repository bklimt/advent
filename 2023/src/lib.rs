pub mod common {
    use anyhow::{Context, Result};
    use std::str::FromStr;

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

    pub trait StrIterator: Iterator {
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
