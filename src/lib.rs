extern crate cli_log;

mod cli;
mod conf;
mod errors;
mod resolver;
mod server;
mod thumb;

// #[global_allocator]
// static ALLOC: leak::LeakingAllocator = leak::LeakingAllocator::new();

pub use {cli::*, conf::*, errors::*, resolver::*, server::*, thumb::*};
