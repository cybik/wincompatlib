pub mod wine;
pub mod proton;

#[cfg(feature = "dxvk")]
pub mod dxvk;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use super::wine::*;
    pub use super::proton::*;

    #[cfg(feature = "dxvk")]
    pub use super::dxvk::*;
}
