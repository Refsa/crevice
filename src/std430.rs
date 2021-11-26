//! Defines traits and types for working with data adhering to GLSL's `std140`
//! layout specification.

mod primitives;
mod sizer;
mod traits;
#[cfg(feature = "std")]
mod writer;
#[cfg(feature = "arrays")]
mod arrays;

pub use crate::bool::Bool;

pub use self::primitives::*;
pub use self::sizer::*;
pub use self::traits::*;
#[cfg(feature = "std")]
pub use self::writer::*;
#[cfg(feature = "arrays")]
pub use self::arrays::Std430ArrayItem;

pub use crevice_derive::AsStd430;
