mod properties_source;

use properties_source::PropertiesSource;
use std::env;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;
use config::{Config, Environment};
use crate::ExtransError;

pub struct Properties {
    pub m_properties: Config,
    m_namespace: String,
    m_class: String,
    m_app: String
}

pub struct PropertiesBuilder {
    m_file_path: Option<String>,
    m_namespace: String,
    m_class: String,
    m_app: String
}

impl PropertiesBuilder {
    /// Creates a new instance of PropertiesBuilder with default namespace, class, and app values.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let builder = PropertiesBuilder::new();
    /// ```
    pub fn new() -> Self {
        PropertiesBuilder {
            m_file_path: None,
            m_namespace: "namespace".to_string(),
            m_class: "class".to_string(),
            m_app: "app".to_string()
        }
    }

    /// Sets the file path for loading properties.
    ///
    /// # Arguments
    /// * `file_path` - The path to the configuration file.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let builder = PropertiesBuilder::new().with_file("config/path");
    /// ```
    pub fn with_file(mut self, file_path: &str) -> Self {
        self.m_file_path = Some(file_path.to_string());
        self
    }

    /// Sets the namespace for the properties.
    ///
    /// # Arguments
    /// * `namespace` - The namespace string.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let builder = PropertiesBuilder::new().with_namespace("my_namespace".to_string());
    /// ```
    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.m_namespace = namespace;
        self
    }

    /// Sets the class for the properties.
    ///
    /// # Arguments
    /// * `cls` - The class string.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let builder = PropertiesBuilder::new().with_class("my_class".to_string());
    /// ```
    pub fn with_class(mut self, cls: String) -> Self {
        self.m_class = cls;
        self
    }

    /// Sets the app identifier for the properties.
    ///
    /// # Arguments
    /// * `app` - The app string.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let builder = PropertiesBuilder::new().with_app("my_app".to_string());
    /// ```
    pub fn with_app(mut self, app: String) -> Self {
        self.m_app = app;
        self
    }

    /// Builds the `Properties` object based on the configured builder settings.
    /// Attempts to load properties from the provided file or $CONFIG_PATH/extrans.properties.
    ///
    /// # Returns
    /// * `Result<Properties, config::ConfigError>` - Returns a `Properties` object or an error if loading fails.
    ///
    /// # Example
    /// ```
    /// use extrans::properties::PropertiesBuilder; 
    /// let properties = PropertiesBuilder::new().with_file("config/path").build();
    /// ```
    pub fn build(self) -> Result<Properties, config::ConfigError> {
        let mut properties = Properties::new(self.m_namespace, self.m_class, self.m_app);
        if let Some(path) = self.m_file_path {
            properties.load_from_file(&path)?;   
        } else {
            let mut config_path: String = env::var("CONFIG_PATH").expect("environment variable CONFIG_PATH not found");        

            if !Path::new(&config_path).exists() {
                return Err(config::ConfigError::NotFound(format!("CONFIG_PATH not found")));
            }

            if config_path.ends_with("/") {
                config_path.push_str("extrans.properties");
            } else {
                config_path.push_str("/extrans.properties");
            }
            properties.load_from_file(&config_path)?;
        }
        Ok(properties)
    }
}

impl Properties {
    /// Creates a new instance of `Properties` with the specified namespace, class, and app.
    /// Typically this should not be called directly but only via the PropertiesBuilder.
    ///
    /// # Arguments
    /// * `ns` - Namespace string.
    /// * `cls` - Class string.
    /// * `app` - App string.
    pub fn new(ns: String, cls: String, app: String) -> Self {
        Properties {
            m_properties: Config::default(),
            m_namespace: ns,
            m_class: cls,
            m_app: app
        }
    }

    /// Loads properties from the specified file path and environment variables.
    /// Typically not called directly, only via PropertiesBuilder.
    ///
    /// # Arguments
    /// * `file_path` - Path to the properties file.
    ///
    /// # Returns
    /// * `Result<(), config::ConfigError>` - Returns `Ok` if successful, or an error if loading fails.
    ///
    pub fn load_from_file(&mut self, file_path: &str) -> Result<(), config::ConfigError> {
        self.m_properties = Config::builder()
            .add_source(PropertiesSource::new(file_path))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        Ok(())
    }

    fn parse_property<T: FromStr>(&self, value: &str) -> Result<T, ExtransError>
    where
    T::Err: Display,
    {
        value.trim_matches(|c| c == '"' || c == '\'').parse::<T>().map_err(|e| ExtransError::ParseError(e.to_string()))
    }

    fn get_property<T: FromStr>(&self, prop: &str) -> Result<T, ExtransError> 
    where
    T::Err: Display,
    {
        let ns_class_app_prop = format!("{}.{}.{}.{}", self.m_namespace, self.m_class, self.m_app, prop);
        let ns_class_prop = format!("{}.{}.{}", self.m_namespace, self.m_class, prop);
        let ns_prop = format!("{}.{}", self.m_namespace, prop);

        if let Some(res) = self.m_properties.get::<String>(&ns_class_app_prop).ok() {
            return self.parse_property::<T>(&res);
        } 

        if let Some(res) = self.m_properties.get::<String>(&ns_class_prop).ok() {
            return self.parse_property::<T>(&res);
        } 

        if let Some(res) = self.m_properties.get::<String>(&ns_prop).ok() {
            return self.parse_property::<T>(&res);
        } 

        if let Some(res) = self.m_properties.get::<String>(&prop).ok() {
            return self.parse_property::<T>(&res);
        } 

        Err(ExtransError::PropertyNotFound(prop.to_string()))
    }

    /// Retrieves a property value based on the configured priority order (namespace.class.app.property, etc.).
    ///
    /// # Arguments
    /// * `prop` - The property name.
    ///
    /// # Returns
    /// * `Result<T, ExtransError>` - Returns the parsed value or an error if the property is not found.
    ///
    /// # Example
    /// ```ignore
    /// let value: i32 = props.get::<i32>("my_property").unwrap();
    /// ```
    pub fn get<T: FromStr>(&self, prop: &str) -> Result<T, ExtransError> 
    where
    T::Err: Display,
    {
        match self.get_property::<T>(prop) {
            Ok(value) => Ok(value),
            Err(_) => Err(ExtransError::PropertyNotFound(prop.to_string())),
        }
    }

    /// Retrieves a property value based on the configured priority order, or returns a default value if not found.
    ///
    /// # Arguments
    /// * `prop` - The property name.
    /// * `default` - Default value to return if the property is not found.
    ///
    /// # Returns
    /// * `T` - The parsed value or the provided default.
    ///
    /// # Example
    /// ```ignore
    /// let value: i32 = props.get_default::<i32>("my_property", 42);
    /// ```
    pub fn get_default<T: FromStr>(&self, prop: &str, default: T) -> T 
    where
    T::Err: Display,
    {
        match self.get_property::<T>(prop) {
            Ok(value) => value,
            Err(_) => default
        }
    }
}