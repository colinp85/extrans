use std::fmt;
use super::field::Field;

pub struct Group {
    m_parent_tag: i32,
    m_first_tag: i32,
    m_tags: Vec<i32>,
    m_fields: Vec<Field>,
}

impl Group {
    pub fn new(parent: i32) -> Self {
        Group {
            m_parent_tag: (parent),
            m_first_tag: (0),
            m_tags: Vec::new(),
            m_fields: Vec::new()
        }
    }

    pub fn get_parent_tag(&self) -> i32 {
        self.m_parent_tag
    }

    pub fn add_field(&mut self, field: Field) {
        if self.m_first_tag == 0 {
            self.m_first_tag = field.get_tag();
        }

        self.m_tags.push(field.get_tag());
        self.m_fields.push(field);
    }

    pub fn is_first_tag(&self, tag: i32) -> bool {
        tag == self.m_first_tag
    }

    pub fn has_tag(&self, tag: i32) -> bool {
        self.m_tags.contains(&tag)
    }

    pub fn iter_fields(&self) -> std::slice::Iter<'_, Field> {
        self.m_fields.iter()
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "group parent tag: {}", self.m_parent_tag)?;
        for field in self.iter_fields() {
            writeln!(f, "    {}", field)?;
        }
        Ok(())
    }
}