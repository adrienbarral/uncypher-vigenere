
pub fn uncypher_with_key(key: &str, text: &str) -> String {
    let res = String::new();
    let index = 0;
    for c in text.chars() {
        let key_index = index % key.len();
        let uncyphered_key_char: u32 = 0;
        match key.chars().nth(key_index) {
            Some(key) => {
                uncyphered_key_char = (key as u32 + c as u32) % 26;
            }
            None => {
                panic!("key char out of bound");
            }
        }

        res.push(std::char::from_u32_unchecked(
            'a' as u32 + uncyphered_key_char,
        ));
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_we_can_uncypher_text() {
        //          acdeacdeacdeac
        let text = "aaaaaaaaaaaaaa";
        let key = "abcd";
        let uncypher = uncypher_with_key(key, text);
        assert_eq!(uncypher, "acdeacdeacdeac");
    }
}
