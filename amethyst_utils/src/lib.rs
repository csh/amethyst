//! A collection of useful amethyst utilities, designed to make your game dev life easier.

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all)]
#![allow(clippy::new_without_default)]

pub use self::app_root_dir::*;

pub mod app_root_dir;
#[cfg(not(feature = "empty"))]
pub mod auto_fov;
pub mod circular_buffer;
pub mod fps_counter;
#[cfg(not(feature = "empty"))]
pub mod ortho_camera;
pub mod removal;
#[cfg(not(feature = "empty"))]
pub mod scene;
pub mod tag;
pub mod time_destroy;
