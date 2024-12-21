// src/storage/mod.rs
mod memory;
mod redis;
mod traits;

pub use memory::MemoryStorage;
pub use redis::RedisStorage;
pub use traits::Storage;
