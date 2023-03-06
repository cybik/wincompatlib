use std::path::PathBuf;

use super::*;

pub trait ProtonWithExt {
    fn with_prefix<T: Into<PathBuf>>(self, prefix: T) -> Self;
    fn with_arch(self, arch: ProtonArch) -> Self;
    fn with_boot<T: Into<PathBuf>>(self, boot: T) -> Self;
    fn with_server<T: Into<PathBuf>>(self, server: T) -> Self;
    fn with_loader(self, loader: ProtonLoader) -> Self;
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

    /// Add wine architecture
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Proton::from_binary("wine")
    ///     .with_arch(ProtonArch::Win64);
    /// ```
    fn with_arch(self, arch: ProtonArch) -> Self {
        Self {
            arch: Some(arch),
            ..self
        }
    }

    /// Add wineboot binary
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Proton::from_binary("wine")
    ///     .with_boot("wineboot");
    /// ```
    fn with_boot<T: Into<PathBuf>>(self, boot: T) -> Self {
        Self {
            wineboot: Some(boot.into()),
            ..self
        }
    }

    /// Add wineserver binary
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Proton::from_binary("wine")
    ///     .with_server("wineserver");
    /// ```
    fn with_server<T: Into<PathBuf>>(self, server: T) -> Self {
        Self {
            wineserver: Some(server.into()),
            ..self
        }
    }

    /// Add wineloader binary
    /// 
    /// ```
    /// use wincompatlib::prelude::*;
    /// 
    /// let wine = Proton::from_binary("wine")
    ///     .with_loader(ProtonLoader::Custom(std::path::PathBuf::from("wine")));
    /// ```
    fn with_loader(self, loader: ProtonLoader) -> Self {
        Self {
            wineloader: loader,
            ..self
        }
    }
}
