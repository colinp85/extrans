use extrans::{FIX, Extrans};

#[test]
fn test_encode() {
    // Create an instance of your implementation.
    let fix = FIX {};
    let result = fix.encode();
    let expected = "Hello".to_string();

    assert_eq!(result, expected);
}

#[test]
fn test_load_dictionary_invalid_file() {
    let mut fix = FIX {};
    let result = fix.load_dictionary("invalid_file.xml");
    assert!(result.is_err(), "Expected an error, but success returned");
}