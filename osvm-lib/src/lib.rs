pub mod oasm;
pub mod opcode;
pub mod osvm;
pub mod preprocessor;
pub mod log;

pub mod utils {
    pub mod defines;
    pub mod error;
    pub mod file;
    pub mod sys_functions;
}

pub mod prelude {
    pub use crate::osvm::*;
    pub use crate::oasm::*;
    pub use crate::utils::file::*;
    pub use crate::log::*;
}
