// src/storage/mod.rs
mod memory;
mod traits;

pub use memory::MemoryStorage;
pub use traits::Storage;
