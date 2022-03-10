mod compiler;
mod macros;
mod mapping;
mod parsing;

// TODO: add integration tests
// TODO: check output wasm code

pub use compiler::Compiler;
pub use mapping::get_mapping;
pub use mapping::Mappings;
pub use mapping::MAPPINGS;
