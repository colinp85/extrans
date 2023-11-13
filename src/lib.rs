pub mod fix;
pub mod extrans_error;

pub use fix::FIX;
pub use extrans_error::ExtransError;

pub trait Extrans {
    fn encode(&self) -> String;
}