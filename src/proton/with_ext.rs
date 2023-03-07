use std::path::PathBuf;

use super::*;

pub trait ProtonWithExt {
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self;
}

impl ProtonWithExt for Proton {
    /// Add path to wine prefix
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Proton::from_binary("wine")
    ///     .with_prefix("/path/to/prefix");
    /// ```
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self {
        Self {
            prefix: Some(prefix.into()),
            ..self
        }
    }
}
