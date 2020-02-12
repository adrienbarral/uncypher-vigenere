extern crate uncypher_vigenere;
use std::path::PathBuf;
use std::fs;

use uncypher_vigenere::key_length_estimator::detect_key_length;
use uncypher_vigenere::cyphered_text_formater::format_cyphered_text;

#[test]
fn test_we_can_detect_key_length() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/cyphered.txt");
    let cyphered_text = fs::read_to_string(d).unwrap();
    let clean_cyphered_text = format_cyphered_text(&cyphered_text);

    let key_length = detect_key_length(&clean_cyphered_text,10);
    
    assert_eq!(key_length, 9);
}