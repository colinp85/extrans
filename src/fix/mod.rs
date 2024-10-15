mod field;
mod group;
mod component;
mod message;
mod dictionary;


use dictionary::Dictionary;
use crate::{Extrans, ExtransError};
//use crate::properties::Properties;
pub struct FIX {
    //m_properties: Properties,
}

impl Extrans for FIX {
    fn encode(&self) -> String {
        "Hello".to_string()
    }
}

impl FIX {
    pub fn new() -> Self {
        FIX {
            //m_properties: Properties;
        }
    }

    pub fn init() {

    }

    pub fn load_dictionary(&mut self, filename: &str) -> Result<(), ExtransError> {
        let mut fix_dict: Dictionary = Dictionary::new();
        fix_dict.load(filename)?;

        Ok(())
    }
}