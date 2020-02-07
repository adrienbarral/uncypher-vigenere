extern crate unidecode;
use unidecode::unidecode;
use regex::Regex;

pub fn format_cyphered_text(text: &str)->String {
    let unidecoded = unidecode(text);
    let re = Regex::new(r"[^A-Za-z]").unwrap();
    let mut result: String = re.replace_all(&unidecoded, "").into();
    result.make_ascii_lowercase();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all_non_letters_characters(){
        let input = "Bonjour Tout le Monde ! C'est à vous !";
        assert_eq!(format_cyphered_text(input), "bonjourtoutlemondecestavous")
    }
}