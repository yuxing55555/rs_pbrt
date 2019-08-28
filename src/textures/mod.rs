//! **Texture** is a template class parameterized by return type of
//! its evaluation function. This design makes it possible to reuse
//! almost all of the code among textures that return different
//! types. PBRT currently uses only **Float** and **Spectrum**
//! textures.
//!
//! - BilerpTexture
//! - Checkerboard2DTexture
//! - ConstantTexture
//! - DotsTexture
//! - FBmTexture
//! - ImageTexture
//! - MarbleTexture
//! - MixTexture
//! - PtexTexture
//! - ScaleTexture
//! - UVTexture
//! - WindyTexture
//! - WrinkledTexture

pub mod checkerboard;
pub mod constant;
pub mod dots;
pub mod fbm;
pub mod imagemap;
pub mod marble;
pub mod mix;
pub mod scale;
pub mod windy;
pub mod wrinkled;
