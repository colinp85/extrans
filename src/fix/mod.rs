pub mod fix_field;

use roxmltree::Document;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::collections::HashMap;
use crate::{Extrans, ExtransError};
use fix_field::FIXField;

pub struct FIX {
    m_fix_fields: HashMap<i32, FIXField>
}

impl Extrans for FIX {
    fn encode(&self) -> String {
        "Hello".to_string()
    }
}

impl FIX {
    pub fn new() -> Self {
        FIX {
            m_fix_fields: HashMap::new()
        }
    }

    fn load_all_fields(&mut self, doc: Document<'_>) -> Result<(), ExtransError> {
        for node in doc.descendants() {
            if node.has_tag_name("fields") {
                for field in node.children() {
                    if field.has_attribute("number") && field.has_attribute("name") {
                        let tag = match i32::from_str(field.attribute("number").unwrap()) {
                            Ok(tag) => tag,
                            Err(e) => {
                                return Err(ExtransError::SetupError(e.to_string()));
                            }
                        };
                        let name = field.attribute("name").unwrap();
                        let field_type = field.attribute("type").unwrap();

                        let f = FIXField::new(tag, String::from(name), String::from(field_type));
                        self.m_fix_fields.insert(tag, f);

                        println!("{} {} {}", tag, name, field_type);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn load_dictionary(&mut self, filename: &str) -> Result<(), ExtransError> {
        let path = Path::new(filename);
        if !path.exists() {
            return Err(ExtransError::SetupError(format!("FIX Dictionary '{}' does not exist.", filename).into()));
        }

        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let doc = match roxmltree::Document::parse(&contents) {
            Ok(doc) => doc,
            Err(e) => { 
                return Err(ExtransError::SetupError(e.to_string()));
            }
        };

        self.load_all_fields(doc)?;

        /*for node in doc.descendants() {
            if node.has_tag_name("header") {
                for field in node.children() {
                    if field.has_attribute("name") && field.has_attribute("required") {
                        let name = field.attribute("name").unwrap();
                        let req = field.attribute("required").unwrap();

                        println!("{} {}", name, req);
                    }
                }
            }
        }*/
        /*let res = Element::from_str(&contents);

        let root;
        match res {
            Ok(contents) => {
                root = contents;
            },
            Err(e) => {
                return Err(ExtransError::SetupError(e.to_string()));
            } 
        }

        println!("iterating xml");
        for child in root.children() {
            if child.is("header", "fix") {
                for field in child.children() {
                    let name = field.get_child("name", "field").unwrap().text();
                    let required = field.get_child("required", "field").unwrap().text();
                    println!("name: {} required: {}", name, required);
                }
            }
        }*/

        Ok(())
    }
}