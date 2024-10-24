use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::Write;

pub type IdoKeyT = i32;
type ItemMap = HashMap<IdoKeyT, IdoItem>;
type IdoArray = Vec<Ido>; 

#[derive(Clone)]
pub struct IdoItem
{
    m_index: u64,
    pub m_key: IdoKeyT,
    pub m_string: String,
    pub m_type: IdoItemType,
    pub m_integer: i64,
    pub m_float: f64,
    pub m_datetime: DateTime<Utc>,
    pub m_array: IdoArray
}

impl IdoItem {
    pub fn new() -> Self {
        IdoItem { 
            m_key: (0),
            m_index: (0),
            m_string: (String::new()),
            m_type: (IdoItemType::STRING),
            m_integer: (0),
            m_float: (0.0),
            m_datetime: (DateTime::<Utc>::MIN_UTC),
            m_array: (IdoArray::new())
        }
    }

    /// Returns the type of the item.
    pub fn get_type(&self) -> IdoItemType {
        self.m_type
    }
    
    /// Returns the item value as a string, if possible.
    ///
    /// # Returns
    ///
    /// - `Some(String)` if the item can be converted to a string.
    /// - `None` if the item type is not convertible to a string.
    pub fn as_string(&self) -> Option<String> {
        match self.m_type {
            IdoItemType::STRING => Some(self.m_string.clone()),
            IdoItemType::FLOAT => Some(self.m_float.to_string()),
            IdoItemType::INTEGER => Some(self.m_integer.to_string()),
            IdoItemType::DATETIME => Some(self.m_datetime.format("%Y-%m-%d %H:%M:%S%.4f").to_string()),
            IdoItemType::ARRAY => Some(format!("<array of {}>", self.m_array.len()))
        }
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum IdoItemType
{
    STRING,
    INTEGER,
    FLOAT,
    DATETIME,
    ARRAY
}

#[derive(Clone)]
pub struct Ido {
    m_items: ItemMap,
    m_idx: u64,
    m_ordered: HashMap<u64, IdoKeyT>
}

pub struct OrderedIdoIterator<'a> {
    m_ido: &'a Ido,
    m_curr: usize
}

impl Iterator for OrderedIdoIterator<'_> {
    type Item = (IdoKeyT, IdoItem);

    fn next(&mut self) -> Option<Self::Item> {
        match self.m_ido.m_ordered.get(&(self.m_curr as u64)) {
            Some(key) => {
                let item = self.m_ido.get_item(&key).unwrap();
                self.m_curr += 1;
                Some((*key, item))
            }
            None => {
                None
            }
        }
    }
}

impl IntoIterator for Ido {
    type Item = (IdoKeyT, IdoItem);
    type IntoIter = IntoIter<IdoKeyT, IdoItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.m_items.into_iter()
    }
}

impl Ido {
    pub fn new() -> Self {
        Ido {
            m_items: ItemMap::new(),
            m_idx: 0,
            m_ordered: HashMap::new(),
        }
    }

    /// Ordered iterator that will iterate the Ido in the order it was populated
    ///
    /// This function should only be used for debug purposes as it will be less
    /// performant than an unordered sort.
    /// 
    /// # Example
    /// 
    /// ```
    /// use extrans::ido::Ido;
    /// 
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, "value1".to_string());
    /// ido.set_string(&2, "value2".to_string());
    /// 
    /// for (key, value) in ido.into_ordered_iterator() {
    ///     match value.as_string() {
    ///         Some(val) => println!("Key: {}, Val: {}", key, val),
    ///         None => println!("Key: {}, Val: None", key),
    ///     }
    /// }
    pub fn into_ordered_iterator(&self) -> OrderedIdoIterator {
        OrderedIdoIterator { m_ido: self, m_curr: 0 }
    }

    /// Clears the Ido object, removing all key-value pairs and resetting internal state.
    ///
    /// This function clears the underlying hashmap, resets the index (`m_idx`) to 0,
    /// and clears the ordered map (`m_ordered`). After calling this function, the Ido
    /// object will be empty with no key-value pairs and will be ready for reuse.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, "value1".to_string());
    /// ido.set_string(&2, "value2".to_string());
    ///
    /// ido.clear();
    /// ```
    pub fn clear(&mut self) {
        self.m_items.clear();
        self.m_ordered.clear();
        self.m_idx = 0;
    }

    /// Returns the number of key-value pairs in the Ido object.
    ///
    /// This function returns the count of key-value pairs stored in the Ido object.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, "value1".to_string());
    /// ido.set_string(&2, "value2".to_string());
    ///
    /// let size = ido.size();
    /// ```
    pub fn size(&self) -> usize {
        self.m_items.len()
    }

    /// Updates the current Ido object with the values from another Ido object.
    ///
    /// This function iterates through the key-value pairs of the `other` Ido object
    /// and inserts or updates the corresponding entries in the current Ido object.
    /// Existing values are replaced with the new values, and new key-value pairs
    /// are added to the current object.
    ///
    /// # Arguments
    ///
    /// * `other` - Another Ido object to update from.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido1 = Ido::new();
    /// ido1.set_string(&1, "value1".to_string());
    /// ido1.set_string(&2, "value2".to_string());
    ///
    /// let mut ido2 = Ido::new();
    /// ido2.set_string(&2, "new_value2".to_string());
    /// ido2.set_string(&3, "value3".to_string());
    ///
    /// ido1.update(&ido2);
    /// ```
    pub fn update(&mut self, other: &Ido) {
        for (key, value) in &other.m_items {
            self.m_items.insert(*key, value.clone());
        }
    }

    /// Checks if the `Ido` object contains the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key (`IdoKeyT`) to be checked.
    ///
    /// # Returns
    ///
    /// Returns `true` if the `Ido` object contains the specified key, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    /// ido.set_integer(&1, 25);
    /// if ido.contains(&1) {
    ///     println!("contains key");
    /// }
    /// ```
    pub fn contains(&self, key: &IdoKeyT) -> bool {
        self.m_items.contains_key(key)
    }

    /// Checks if the value associated with the given key has the specified type.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key for which the type needs to be checked.
    /// * `ty` - A reference to the `IdoItemType` enum representing the expected type.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the value associated with the key has the expected type.
    /// - `true` if the value has the expected type.
    /// - `false` if the value does not exist or has a different type.
    pub fn is_type(&self, key: &IdoKeyT, ty: &IdoItemType) -> bool
    {
        if let Some(value) = self.m_items.get(key) {
            return value.m_type == *ty;
        } else {
            false       
        }
    }

    /// Sets an item with the specified key in the internal storage.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key associated with the item.
    /// * `item` - The item to be set.
    ///
    /// # Remarks
    ///
    /// This function assigns the given item to the specified key in the internal storage.
    /// It updates the item's key, assigns an index, and inserts the item into the storage map and ordered set.
    /// The index is incremented to maintain ordering.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido, IdoKeyT, IdoItem};
    ///
    /// let mut ido = Ido::new();
    /// let item = IdoItem::new();
    ///
    /// ido.set_item(&42, item);
    /// ```
    pub fn set_item(&mut self, key: &IdoKeyT, mut item: IdoItem)
    {
        item.m_key = *key;
        item.m_index = self.m_idx;

        if let Some(value) = self.m_items.get(key) {
            self.m_ordered.remove(&value.m_index);
        }

        self.m_items.insert(*key, item);
        self.m_ordered.insert(self.m_idx, *key);
        
        self.m_idx += 1;
    }

    /// Sets a string value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value.
    /// * `val` - The string value to be set.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, "John Doe".to_string());
    /// ```
    pub fn set_string (&mut self, key: &IdoKeyT, val: String)
    {
        let mut item: IdoItem = IdoItem::new();
        item.m_type = IdoItemType::STRING;
        item.m_string = val;

        self.set_item(key, item);
    }

    /// Sets an integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value.
    /// * `val` - The integer value to be set.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let mut ido = Ido::new();
    /// ido.set_integer(&1, 42);
    /// ```
    pub fn set_integer (&mut self, key: &IdoKeyT, val: i64)
    {
        let mut item: IdoItem = IdoItem::new();
        item.m_type = IdoItemType::INTEGER;
        item.m_integer = val;

        self.set_item(key, item);
    }

    /// Sets a floating-point value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value.
    /// * `val` - The floating-point value to be set.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let mut ido = Ido::new();
    /// ido.set_f64(&1, 3.14159);
    /// ```
    pub fn set_f64 (&mut self, key: &IdoKeyT, val: f64)
    {
        let mut item: IdoItem = IdoItem::new();
        item.m_type = IdoItemType::FLOAT;
        item.m_float = val;

        self.set_item(key, item);
    }

    /// Retrieves an item from the collection using the specified key.
    ///
    /// If an item is found in the collection associated with the provided key, a clone of the item is returned within a `Some` variant.
    /// If no item is found, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `key`: A reference to a key of type `IdoKeyT` used for item lookup.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    ///
    /// let mut collection = Ido::new();
    /// let key = 1;
    /// collection.set_integer(&1, 100);
    /// let item = collection.get_item(&key);
    ///
    /// if let Some(found_item) = item {
    ///     println!("Item found: {:?}", found_item.as_string());
    /// } else {
    ///     println!("Item not found for key: {:?}", key);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function returns a clone of the found item to ensure the original collection remains unchanged.
    ///
    pub fn get_item(&self, key: &IdoKeyT) -> Option<IdoItem> {
        if let Some(value) = self.m_items.get(key) {
            Some(value.clone())
        } else {
            return None;
        }
    }

    /// Retrieves a string value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value (integer).
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the string value if it exists, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(string_value) = ido.get_string(&42) {
    ///     println!("Value: {}", string_value);
    /// } else {
    ///     println!("Value not found.");
    /// }
    /// ```
    pub fn get_string(&self, key: &IdoKeyT) -> Option<String> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::STRING {
                return None;
            } else {
                Some(value.m_string.clone())
            }
        } else {
            return None;
        }
    }

    /// Retrieves a 64-bit signed integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 64-bit signed integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let mut ido = Ido::new();
    /// ido.set_integer(&42, 100);
    /// if let Some(value) = ido.get_i64(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_i64(&self, key: &IdoKeyT) -> Option<i64> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer)
            }
        } else {
            None
        }
    }

    /// Retrieves a 32-bit signed integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 32-bit signed integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_i32(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_i32(&self, key: &IdoKeyT) -> Option<i32> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as i32)
            }
        } else {
            None
        }
    }

    /// Retrieves a 16-bit signed integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 16-bit signed integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_i16(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_i16(&self, key: &IdoKeyT) -> Option<i16> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as i16)
            }
        } else {
            None
        }
    }

    /// Retrieves an 8-bit signed integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 8-bit signed integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_i8(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_i8(&self, key: &IdoKeyT) -> Option<i8> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as i8)
            }
        } else {
            None
        }
    }

    /// Retrieves a 64-bit unsigned integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 64-bit unsigned integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_u64(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_u64(&self, key: &IdoKeyT) -> Option<u64> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as u64)
            }
        } else {
            None
        }
    }

    /// Retrieves a 32-bit unsigned integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 32-bit unsigned integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_u32(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_u32(&self, key: &IdoKeyT) -> Option<u32> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as u32)
            }
        } else {
            None
        }
    }

    /// Retrieves a 16-bit unsigned integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 16-bit unsigned integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_u16(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_u16(&self, key: &IdoKeyT) -> Option<u16> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as u16)
            }
        } else {
            None
        }
    }

    /// Retrieves an 8-bit unsigned integer value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 8-bit unsigned integer value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_u8(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not an integer.");
    /// }
    /// ```
    pub fn get_u8(&self, key: &IdoKeyT) -> Option<u8> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::INTEGER {
                return None;
            } else {
                Some(value.m_integer as u8)
            }
        } else {
            None
        }
    }

    /// Retrieves a 64-bit floating-point value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key (integer) associated with the value.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing the 64-bit floating-point value if it exists and is of the correct type, or `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::{Ido};
    /// let ido = Ido::new();
    /// if let Some(value) = ido.get_f64(&42) {
    ///     println!("Value: {}", value);
    /// } else {
    ///     println!("Value not found or not a floating-point number.");
    /// }
    /// ```
    pub fn get_f64(&self, key: &IdoKeyT) -> Option<f64> {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::FLOAT {
                return None;
            } else {
                Some(value.m_float)
            }
        } else {
            None
        }
    }

    /// Empties the array associated with the given key, or creates a new empty array if the key doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key of the array in the Ido object.
    fn empty_array(&mut self, key: &IdoKeyT)
    {
        let mut item: IdoItem = IdoItem::new();
        item.m_type = IdoItemType::ARRAY;
        item.m_array = IdoArray::new();
        self.set_item(key, item);
    }

    /// Appends an Ido object to an array within the Ido object.
    ///
    /// If the specified key does not exist or the value associated with the key is not an array,
    /// a new array is created and assigned to the key.
    /// The provided Ido object is then appended to the array.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key identifying the array within the Ido object.
    /// * `data` - The Ido object to append to the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    ///
    /// let mut data = Ido::new();
    /// data.set_string(&1, String::from("John"));
    /// data.set_integer(&2, 30);
    ///
    /// ido.append_array(&11, data);
    /// ```
    pub fn append_array(&mut self, key: &IdoKeyT, data: Ido)
    {
        if let Some(value) = self.m_items.get(key) {
            if value.m_type != IdoItemType::ARRAY {
                self.empty_array(key);
            }
        }
        else {
            self.empty_array(key);
        }

        if let Some(value) = self.m_items.get_mut(key) {
            (*value).m_type = IdoItemType::ARRAY;
            (*value).m_array.push(data);
        }
    }

    /// Deletes an item from the Ido object based on the given key.
    ///
    /// If an item with the specified key exists in the Ido object, it will be removed.
    /// If no item exists with the given key, this function does nothing.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the item to be deleted.
    ///
    /// # Example
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, "value1".to_string());
    /// ido.set_string(&2, "value2".to_string());
    ///
    /// ido.delete_item(&1);
    /// ```
    pub fn delete_item(&mut self, key: &IdoKeyT) {
        if let Some(value) = self.m_items.get(key) {
            self.m_ordered.remove(&value.m_index);
            self.m_items.remove(key);
        }
    }

    /// Converts the Ido object to a string representation.
    ///
    /// Returns a string that represents the Ido object, with key-value pairs separated by commas.
    /// The key-value pairs are sorted based on the order of insertion.
    ///
    /// # Examples
    ///
    /// ```
    /// use extrans::ido::Ido;
    ///
    /// let mut ido = Ido::new();
    /// ido.set_string(&1, String::from("John"));
    /// ido.set_integer(&2, 30);
    /// ido.set_string(&3, String::from("New York"));
    ///
    /// let result = ido.to_string();
    /// ```
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut count = 0;

        for (key, value) in self.into_ordered_iterator() {
            if count != 0 {
                result.push(',');
            }

            if let Some(value_str) = value.as_string() {
                if value.get_type() == IdoItemType::ARRAY {
                    for array_item in &value.m_array {
                        write!(result, "{}=[{}]", key, array_item.to_string()).unwrap();
                    }
                } else {
                    write!(result, "{}={}", key, value_str).unwrap();
                }
            } else {
                continue
            }

            count += 1;
        }
        result
    }
}