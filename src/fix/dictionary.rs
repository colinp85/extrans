use crate::ExtransError;

use super::field::Field;
use super::component::{Component, ComponentFieldType};
use super::message::Message;
use super::group::Group;

use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

pub struct Dictionary {
    m_fix_fields: HashMap<i32, Field>,
    m_field_to_tag: HashMap<String, i32>,
    m_field_to_type: HashMap<String, String>,
    m_components: HashMap<String, Component>,
    m_messages: HashMap<String, Message>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            m_fix_fields: HashMap::new(),
            m_field_to_tag: HashMap::new(),
            m_field_to_type: HashMap::new(),
            m_components: HashMap::new(),
            m_messages: HashMap::new(),
        }
    }

    pub fn get_field_by_name(&self, name: &String) -> Option<&Field> {
        if let Some(tag) = self.m_field_to_tag.get(name) {
            Some(self.m_fix_fields.get(tag).unwrap())
        } else {
            None
        }
    }

    fn load_fields(&mut self, node: &roxmltree::Node) -> Result<(), ExtransError> {
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

                let f = Field::new(tag, String::from(name), String::from(field_type));
                self.m_fix_fields.insert(tag, f);

                self.m_field_to_tag.insert(name.to_string(), tag);
                self.m_field_to_type.insert(name.to_string(), field_type.to_string());
            }
        }
        Ok(())
    }

    fn load_component(&mut self, components: &roxmltree::Node) -> Result<(), ExtransError> {
        for component_node in components.children() {
            if component_node.has_attribute("name") {
                let component_name = component_node.attribute("name").unwrap().to_string();
                let mut component = Component::new(component_name.clone());
                for field_node in component_node.children() {
                    match field_node.tag_name().name() {
                        "field" => component.add_field(field_node.attribute("name").unwrap().to_string()),
                        "component" => component.add_component(field_node.attribute("name").unwrap().to_string()),
                        "group" => {
                            let mut group_fields: Vec<String> = Vec::new();
                            for node in field_node.children() {
                                if node.has_attribute("name") {
                                    group_fields.push(node.attribute("name").unwrap().to_string());
                                }
                            }
                            component.add_group(field_node.attribute("name").unwrap().to_string(), group_fields);
                        },
                        _ => {},
                    }
                }
                self.m_components.insert(component_name, component);
            }
        }
        Ok(())
    }

    fn load_messages(&mut self, messages: &roxmltree::Node) -> Result<(), ExtransError> {
        for message_node in messages.children() {
            if message_node.has_attribute("name") && message_node.has_attribute("msgtype") && message_node.has_attribute("msgcat") {
                let message_name = message_node.attribute("name").unwrap().to_string();
                let message_type = message_node.attribute("msgtype").unwrap().to_string();
                let message_cat = message_node.attribute("msgcat").unwrap().to_string();
                
                let mut message: Message = Message::new(message_name, message_type.clone(), message_cat);

                for node in message_node.children() {
                    if node.has_tag_name("field") {
                        if node.has_attribute("name") {
                            let field_name = node.attribute("name").unwrap();
                            if let Some(field) = self.get_field_by_name(&field_name.to_string()) {
                                message.add_field(field.clone());
                            }
                        }
                    } else if node.has_tag_name("component") {
                        if node.has_attribute("name") {
                            let component_name = node.attribute("name").unwrap();
                            if let Some(component) = self.m_components.get(component_name) {
                                self.resolve_component(&mut message, component);
                            }
                        }
                    } else if node.has_tag_name("group") {
                        if node.has_attribute("name") {
                            let group_name = node.attribute("name").unwrap();
                            if let Some(field) = self.get_field_by_name(&group_name.to_string()) {
                                let mut group: Group = Group::new(field.get_tag());
                                for field_node in node.children() {
                                    if field_node.has_tag_name("field") && field_node.has_attribute("name") {
                                        let field_name = field_node.attribute("name").unwrap().to_string();
                                        if let Some(field) = self.get_field_by_name(&field_name) {
                                            group.add_field(field.clone());
                                        }
                                    }
                                }
                                message.add_group(group);
                            }
                        }
                    }
                }
                println!("message: {}", message);
                self.m_messages.insert(message_type.clone(), message);
            }
        }
        Ok(())
    }

    fn resolve_component(&self, message: &mut Message, component: &Component) {
        for (cft, field_name) in component.iter() {
            match cft {
                ComponentFieldType::FIELD => {
                    if let Some(field) = self.get_field_by_name(&field_name.to_string()) {
                        message.add_field(field.clone());
                    }
                },
                ComponentFieldType::GROUP => {
                    if let Some(field) = self.get_field_by_name(&field_name.to_string()) {
                        let mut group: Group = Group::new(field.get_tag());
                        if let Some(group_fields) = component.get_group_fields(field_name.to_string()) {
                            for gf in group_fields {
                                if let Some(field) = self.get_field_by_name(&gf.to_string()) {
                                    group.add_field(field.clone());
                                }
                            }
                        }
                        message.add_group(group);
                    }
                },
                ComponentFieldType::COMPONENT => {
                    if let Some(lookup) = self.m_components.get(field_name) {
                        self.resolve_component(message, lookup);
                    }
                }
            }
        } 
    }

    pub fn load(&mut self, filename: &str) -> Result<(), ExtransError> {
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

        for node in doc.descendants() {
            if node.has_tag_name("components") {
                self.load_component(&node).map_err(|e| ExtransError::SetupError(e.to_string()))?;
            } else if node.has_tag_name("fields") {
                self.load_fields(&node).map_err(|e| ExtransError::SetupError(e.to_string()))?;
            }
        }

        for node in doc.descendants() {
            if node.has_tag_name("messages") {
                self.load_messages(&node).map_err(|e| ExtransError::SetupError(e.to_string()))?;
            }
        }

        Ok(())

    }
}