pub mod traits;
pub mod errors;

pub type Result<T> = std::result::Result<T, self::errors::NumericalError>;
