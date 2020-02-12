pub fn uncypher_with_key(key: &str, text: &str) -> String {
    let mut res = String::new();
    /* let index = 0;
    let key_letters = key.chars().map(|c| c as u32 - 'a' as u32).collect();

    for char_to_cypher in text.chars() {
        let letter_to_cypher = char_to_cypher as u32 - 'a' as u32;  // Between 0 and 26
        let key_index = index % key.len();
        //let key_char: u32;
        if let Some(key_char) = key.chars().nth(key_index) {
            let cyphered_key_char = (key_char as u32 + c as u32) % 26;
            if let Some(c) = std::char::from_u32('a' as u32 + uncyphered_key_char) {
                res.push(c);
            }
        }
    }*/
    return res;
}

fn convert_to_letter_index(text: &str) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for c in text.chars() {
        res.push((c as u32 - 'a' as u32) as u8);
    }
    return res;
}

fn convert_from_letter_index(text: &Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    let text_as_ascii: Vec<u8> = text.as_slice().iter().map(|c| c + ('a' as u8)).collect();
    return String::from_utf8(text_as_ascii);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_we_can_convert_to_letter_index_in_alphabet() {
        let text = "abcdefzz";
        let res = convert_to_letter_index(text);
        assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 25, 25]);
    }

    #[test]
    fn test_we_can_convert_from_letter_index_in_alphabet() {
        let text: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 25, 25];
        let res = convert_from_letter_index(&text);
        match res {
            Ok(result_text) => assert_eq!(result_text, "abcdefzz"),
            Err(_) => assert_eq!(true, false),
        }
    }

    #[test]
    fn test_we_can_uncypher_text() {
        //          acdeacdeacdeac
        let text = "aaaaaaaaaaaaaa";
        let key = "abcd";
        let uncypher = uncypher_with_key(key, text);
        assert_eq!(uncypher, "acdeacdeacdeac");
    }
}
