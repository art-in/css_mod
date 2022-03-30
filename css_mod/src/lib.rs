#![warn(clippy::unwrap_used)]

mod compiler;
mod macros;
mod mapping;
mod parsing;

// TODO: add integration tests
// TODO: manually test on windows

pub use compiler::Compiler;
pub use mapping::get_mapping;
pub use mapping::Mappings;
pub use mapping::MAPPINGS;
