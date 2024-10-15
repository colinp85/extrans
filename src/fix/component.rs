use std::collections::HashMap;
use std::slice::Iter;
use std::fmt;

pub enum ComponentFieldType {
    FIELD,
    COMPONENT,
    GROUP
}

impl fmt::Display for ComponentFieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &ComponentFieldType::FIELD => "Field",
            &ComponentFieldType::COMPONENT => "Component",
            &ComponentFieldType::GROUP => "Group",
        };
        write!(f, "{}", s)
    }
}

pub struct Component {
    _m_name: String,
    m_fields: Vec<(ComponentFieldType, String)>, 
    m_groups: HashMap<String, Vec<String>>,
}

impl Component {
    pub fn new(name: String) -> Self {
        Component {
            _m_name: (name),
            m_fields: Vec::new(),
            m_groups: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, name: String) {
        self.m_fields.push((ComponentFieldType::FIELD, name))
    }

    pub fn add_component(&mut self, name: String) {
        self.m_fields.push((ComponentFieldType::COMPONENT, name))
    }

    pub fn add_group(&mut self, name: String, group_fields: Vec<String>) {
        self.m_fields.push((ComponentFieldType::GROUP, name.clone()));
        self.m_groups.insert(name, group_fields);
    }

    pub fn set_fields(&mut self, fields: Vec<(ComponentFieldType, String)>) {
        self.m_fields = fields;
    }

    pub fn iter(&self) -> Iter<(ComponentFieldType, String)> {
        self.m_fields.iter()
    }

    pub fn get_group_fields(&self, group_name: String) -> Option<Vec<String>> {
        if let Some(fields) = self.m_groups.get(&group_name) {
            Some(fields.clone())
        } else {
            None
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (t, n)in self.m_fields.iter() {
            writeln!(f, "{} {}", t, n)?;
        } 
        Ok(())
    }
}