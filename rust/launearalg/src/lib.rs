//! # launearalg
//!
//! `launearalg` is a basic implementation of a 2D matrix, and gaussian-elimination solver.
//! Its API is designed for convenience over raw performance, and applications perferring a
//! lighter footprint should consider the [`ndarray`](https://crates.io/ndarray) crate.
//!
//! `launearalg` makes an opinionated trade off; choosing to use double the memory and a
//! highly ineffecient Vec<Vec<T>> internal representations in return for a clean, natural
//! API (I hope).

pub mod interpolater;
pub mod matrix;
pub mod solver;
pub mod traits;
pub mod vector;

pub mod impl_slice;
mod impl_vec_vec;
mod impl_vec_vector;
