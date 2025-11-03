use std::fmt;

#[derive(Debug)]
pub enum NumericalError {
    ConvergenceError { context: String },
    LengthError { context: String },
    MultipleRootError { context: String },
    InvalidArgument(String),
}

impl fmt::Display for NumericalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumericalError::ConvergenceError { context } => {
                write!(
                    f,
                    "Maximum steps exceeded without convergence in {}",
                    context
                )
            }
            NumericalError::LengthError { context } => {
                write!(
                    f,
                    "Length is mismatch in {}", 
                    context
                )
            }
            NumericalError::MultipleRootError { context } => {
                write!(
                    f,
                    "Multiple root are in {}", 
                    context
                )
            }
            NumericalError::InvalidArgument(msg) => {
                write!(f, "Invalid argument: {}", msg)
            }
        }
    }
}

impl std::error::Error for NumericalError {}
