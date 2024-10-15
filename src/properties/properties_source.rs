
use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};
use config::{Source, Map, Value, ConfigError};

#[derive(Debug, Clone)]
pub struct PropertiesSource {
    file_path: String,
}

impl PropertiesSource {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl Source for PropertiesSource {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let file = match FsFile::open(&self.file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(ConfigError::Foreign(Box::new(e)));
            }
        };
        
        let reader = BufReader::new(file);
        let mut map = Map::new();

        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    let line = line.trim();

                    if !line.is_empty() && !line.starts_with('#') && !line.starts_with('!') {
                        if let Some((key, value)) = line.split_once('=') {
                            let key = key.trim();//.replace('.', "__"); // Convert to config-rs nested format
                            let value = value.trim().to_string();

                            map.insert(key.to_string(), Value::from(value));
                        }
                    }
                }
                Err(e) => {
                    return Err(ConfigError::Foreign(Box::new(e)));
                }
            }
        }

        Ok(map)
    }
}
