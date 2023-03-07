use std::path::PathBuf;

use super::*;

pub trait SteamWithExt {
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self;
}

impl SteamWithExt for Steam {
    /// Add path to wine prefix
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Steam::from_binary("wine")
    ///     .with_prefix("/path/to/prefix");
    /// ```
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self {
        Self {
            prefix: Some(prefix.into()),
            ..self
        }
    }
}
