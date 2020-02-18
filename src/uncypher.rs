pub fn uncypher_with_key(key: &str, text: &str) -> Result<String, std::string::FromUtf8Error> {

    let letters_index_text = convert_to_letter_index(text);
    let letters_index_key = convert_to_letter_index(key);
    let uncyphered = uncypher_with_key_as_letters_index(&letters_index_key, &letters_index_text);
    return convert_from_letter_index(&uncyphered);
}

pub fn uncypher_with_key_as_letters_index(key: &Vec<u8>, text: &Vec<u8>) -> Vec<u8> {
    let mut res_as_letters_index: Vec<u8> = Vec::with_capacity(text.len());

    for (index, char_to_cypher) in text.iter().enumerate() {
        let key_index = index % key.len();
        if let Some(key_char) = key.get(key_index) {
            res_as_letters_index.push((char_to_cypher + key_char)%26);
        }
    }
    return res_as_letters_index;
}

pub fn convert_to_letter_index(text: &str) -> Vec<u8> {
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
        let text = "cleartext";
        //         +abcdabcda
        //         =cmgdrugat
        let key = "abcd";
        if let Ok(uncypher) = uncypher_with_key(key, text){
            assert_eq!(uncypher, "cmgdrugat");
        }else {
            assert_eq!(false, true);
        }
    }
}
