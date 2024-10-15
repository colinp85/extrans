/// The `FIXFieldType` enum represents different field types used in the FIX (Financial Information eXchange) protocol.
/// FIX fields can have various data types, and this enum provides a clear way to represent them. 
/// Some of the types have been normalised where underlying data type can be captured by a single type.
///
/// # Variants
///
/// - `INTEGER`: Represents a 32-bit signed integer. INT, LENGTH and DAYOFMONTH have been mapped to INTEGER.
/// - `DOUBLE`: Represents a double-precision floating-point number. PRICE, AMT, FLOAT, PRIECOFFSET, QTY have been mapped to DOUBLE.
/// - `STRING`: Represents a sequence of characters.
/// - `UTCTIMESTAMP`: Represents a timestamp in the UTC (Coordinated Universal Time) format.
/// - `MSTIMESTAMP`: Represents a timestamp with milliseconds.
/// - `UTCDATE`: Represents a date in the UTC format.
/// - `UTCTIMEONLY`: Represents a time in the UTC format.
/// - `MONTHYEAR`: Represents a month and year.
/// - `LOCALMKTDATE`: Represents a date in the local market's timezone.
///
/// This enum is not intended to be used elsewhere but will serve as a performance/convenience mechanism for 
/// field setters/getters.

use std::fmt;

#[derive(Clone)]
enum FieldType {
    INTEGER,
    DOUBLE,
    STRING,
    UTCTIMESTAMP,
    MSTIMESTAMP,
    UTCDATE,
    UTCTIMEONLY,
    MONTHYEAR,
    LOCALMKTDATE
}

#[derive(Clone)]
pub struct Field {
    m_tag: i32,
    m_name: String,
    m_field_type: FieldType,
}

fn string_to_field_type(value: &str) -> FieldType {
    match value {
        "INT" | "LENGTH" | "DAYOFMONTH" => FieldType::INTEGER,
        "QTY" | "FLOAT" | "PRICE" | "PRICEOFFSET" | "AMT" => FieldType::DOUBLE,
        "UTCTIMESTAMP" => FieldType::UTCTIMESTAMP,
        "MSTIMESTAMP" => FieldType::MSTIMESTAMP,
        "UTCDATE" => FieldType::UTCDATE,
        "UTCTIMEONLY" => FieldType::UTCTIMEONLY,
        "MONTHYEAR" => FieldType::MONTHYEAR,
        "LOCALMKTDATE" => FieldType::LOCALMKTDATE,
        _ => FieldType::STRING // default to STRING
    }
}

impl Field {
    pub fn new(tag: i32, name: String, ftype: String) -> Self {
        Field { 
            m_tag: (tag),
            m_name: (name),
            m_field_type: (string_to_field_type(ftype.as_str()))
        }
    }

    pub fn get_tag(&self) -> i32 {
        self.m_tag
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldType::INTEGER => write!(f, "INTEGER")?,
            _ => write!(f, "STRING")?,
        }
        Ok(())
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "field: {} {} {}", self.m_tag, self.m_name, self.m_field_type)?;
        Ok(())
    }
}