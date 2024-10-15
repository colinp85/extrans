use std::collections::HashMap;
use std::fmt;

use super::field::Field;
use super::group::Group;

pub struct Message {
    m_name: String,
    m_type: String,
    m_cat: String,
    m_fields: HashMap<i32, Field>,
    m_groups: HashMap<i32, Group>,
}

impl Message {
    pub fn new(name: String, msgtype: String, msgcat: String) -> Self {
        Message {
            m_name: name,
            m_type: msgtype,
            m_cat:msgcat,
            m_fields: HashMap::new(),
            m_groups: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) {
        self.m_fields.insert(field.get_tag(), field);
    }

    pub fn add_group(&mut self, group: Group) {
        self.m_groups.insert(group.get_parent_tag(), group);
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", self.m_name, self.m_type, self.m_cat)?;
        for (_t, n)in self.m_fields.iter() {
            writeln!(f, "    {}", n)?;
        } 

        for (_t, g) in self.m_groups.iter() {
            writeln!(f, "    {}", g)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}