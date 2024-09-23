#[derive(PartialEq)]
pub enum Error {
    None = 0,
    
    RegisterOverflow,
    RegisterUnderflow,
    StackOverflow,
    StackUnderflow,
    
    InvalidOpcodeAccess,
    InvalidOperand,
    InvalidRegister,
    InvalidSection,
    InvalidSysFunction,
    
    ErrIllegalMemoryAccess,
    
    DivByZero,
}

impl Error {
    pub fn as_string(self: &Self) -> String {
        match self {
            Error::None => return "None".to_string(),
            
            Error::RegisterOverflow => return "RegisterOverflow".to_string(),
            Error::RegisterUnderflow => return "RegisterUnderflow".to_string(),
            Error::StackOverflow => return "StackOverflow".to_string(),
            Error::StackUnderflow => return "StackUnderflow".to_string(),
            
            Error::InvalidOpcodeAccess => return "InvalidOpcodeAccess".to_string(),
            Error::InvalidOperand => return "InvalidOperand".to_string(),
            Error::InvalidRegister => return "InvalidRegister".to_string(),
            Error::InvalidSection => return "InvalidSection".to_string(),
            Error::InvalidSysFunction => return "InvalidSysFunction".to_string(),
            
            Error::ErrIllegalMemoryAccess => return "ErrIllegalMemoryAccess".to_string(),
            
            Error::DivByZero => return "DivByZero".to_string(),
        }
    }
}