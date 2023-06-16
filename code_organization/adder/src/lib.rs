//! # Art
//!
//! A library for modelling artistic concepts.

// re-export so that users of this crate can just do `crate::PrimaryColor`
pub use self::kinds::*;
pub use self::utils::*;

pub mod kinds {
    /// The primary colors according to the RYB model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    /// The secondary colors according to the RYB model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
