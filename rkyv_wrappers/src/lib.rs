//! Accessory and community-contributed wrapper types for [rkyv](https://github.com/rkyv/rkyv).

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

pub mod as_hashmap;
pub mod custom_phantom;

#[cfg(test)]
pub mod tests;
