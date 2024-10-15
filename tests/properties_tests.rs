#[cfg(test)]
mod tests {
    use extrans::properties::{Properties, PropertiesBuilder};

    fn setup_test_config() -> Result<Properties, config::ConfigError> {
        let config = PropertiesBuilder::new().with_file("tests/config/extrans.properties").build()?;
        Ok(config)
    }

    #[test]
    fn test_get_existing_string_property() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get::<String>("extrans.properties.fix_dictionary").unwrap();
        assert_eq!(value, "DictionaryValue");
        Ok(())
    }

    #[test]
    fn test_get_default_string() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get_default::<String>("extrans.properties.not_exist", "hello".to_string());
        assert_eq!(value, "hello");
        Ok(())
    }

    #[test]
    fn test_get_existing_integer_property() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get::<i32>("extrans.properties.max_connections").unwrap();
        assert_eq!(value, 10);
        Ok(())
    }

    #[test]
    fn test_get_existing_boolean_property() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get::<bool>("extrans.properties.enabled").unwrap();
        assert!(value);
        Ok(())
    }

    #[test]
    fn test_get_existing_float_property() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get::<f64>("extrans.properties.timeout").unwrap();
        assert_eq!(value, 5.5);
        Ok(())
    }

    #[test]
    fn test_get_missing_property_with_default() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let value = config.get_default::<i32>("extrans.properties.non_existent", 100);
        assert_eq!(value, 100);
        Ok(())
    }

    #[test]
    fn test_get_fallback_to_default_on_parse_error() -> Result<(), config::ConfigError>  {
        let config = setup_test_config()?;
        let value = config.get_default::<i32>("extrans.properties.fix_dictionary", 100); // Attempting to parse a string as an i32
        assert_eq!(value, 100);
        Ok(())
    }

    #[test]
    fn test_get_missing_property_error() -> Result<(), config::ConfigError> {
        let config = setup_test_config()?;
        let result = config.get::<String>("extrans.properties.non_existent");
        assert!(result.is_err());
        Ok(())
    }
}