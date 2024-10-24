pub mod ido;
pub mod fix;
pub mod properties;
pub mod extrans_error;
pub mod log_builder;

pub use fix::FIX;
pub use extrans_error::ExtransError;

pub trait Extrans {
    fn encode(&self) -> String;
}