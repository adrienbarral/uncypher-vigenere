extern crate ndarray;
use crate::uncypher::convert_to_letter_index;
use crate::uncypher::uncypher_with_key_as_letters_index;
use ndarray::Array1;

pub type Alphabet = Array1<u32>;
pub type AlphabetNormalized = Array1<f32>;

pub fn find_key(key_length: usize, text: &str, language: &AlphabetNormalized) -> String {
    let mut key = String::new();
    for index in 0..key_length {
        let c = std::char::from_u32(
            find_most_probable_key_letter(index, key_length, text, language) as u32,
        );
        match c {
            Some(c) => key.push(c),
            None => panic!("Key length out of bound"),
        }
    }
    return key;
}

fn kullback_lieber_divergence(p: &AlphabetNormalized, q: &AlphabetNormalized) -> f32 {
    assert_eq!(p.len(), q.len());
    let mut res: f32 = 0.0;

    for index in 0..p.len() {
        res = res + p[index] * f32::log10(p[index] / q[index]);
    }
    return res;
}

fn find_most_probable_key_letter(
    offset: usize,
    key_length: usize,
    text: &str,
    language: &AlphabetNormalized,
) -> u8 {
    let tmp = extract_all_letters_spaced_by(text, offset, key_length);
    let all_letters_cyphered_by_this_key_letter = convert_to_letter_index(&tmp);

    let mut most_probable = ('a' as u8, std::f32::MAX);
    for candidate in 0..26 {
        let candidate_slice = vec![candidate];
        let uncyphered = uncypher_with_key_as_letters_index(
            &candidate_slice,
            &all_letters_cyphered_by_this_key_letter,
        );

        let histogram = get_histogram_of_letter_occurance(&uncyphered);
        let divergence_to_language_histogram = kullback_lieber_divergence(&histogram, language);

        if divergence_to_language_histogram < most_probable.1 {
            most_probable.0 = candidate_char;
            most_probable.1 = divergence_to_language_histogram;
        }
    }

    return most_probable.0;
}

fn extract_all_letters_spaced_by(text: &str, offset: usize, space: usize) -> String {
    assert!(space > 0);

    let mut res = std::string::String::new();
    let mut index = offset;
    while index < text.len() {
        if let Some(c) = text.chars().nth(index) {
            res.push(c);
        }
        index += space - 1;
    }
    return res;
}

fn get_histogram_of_letter_occurance(text: &Vec<u8>) -> AlphabetNormalized {
    let mut occurances = Alphabet::zeros(26);

    for c in text.iter() {
        let index = *c as usize;
        occurances[index] = occurances[index] + 1;
    }

    return normalize(&occurances);
}

fn normalize(vector: &Alphabet) -> AlphabetNormalized {
    let sum = vector.sum() as f32;
    let mut res = AlphabetNormalized::zeros(vector.len());
    for i in 0..vector.len() {
        res[i] = vector[i] as f32 / sum;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uncypher;
    use crate::languages_letters_frequency;

    #[test]
    fn test_we_can_get_histogram_of_letter_occurance() {
        let text = "abcdefaaaa";
        let text_as_u8 = uncypher::convert_to_letter_index(text);
        let histogram = get_histogram_of_letter_occurance(&text_as_u8);
        assert_eq!(histogram.len(), 26);
        assert_eq!(histogram[0], 5. / 10.);
        assert_eq!(histogram[1], 1. / 10.);
        assert_eq!(histogram[2], 1. / 10.);
        assert_eq!(histogram[3], 1. / 10.);
        assert_eq!(histogram[4], 1. / 10.);
        assert_eq!(histogram[5], 1. / 10.);
        assert_eq!(histogram[6], 0.);
    }

    #[test]
    fn test_we_can_get_all_letter_spaced() {
        //          0123456789
        let text = "abcdeabcdeabcdea";
        assert_eq!(extract_all_letters_spaced_by(&text, 0, 6), "aaaa");
        assert_eq!(extract_all_letters_spaced_by(&text, 1, 6), "bbb");
        assert_eq!(extract_all_letters_spaced_by(&text, 2, 6), "ccc");
        assert_eq!(extract_all_letters_spaced_by(&text, 3, 6), "ddd");
        assert_eq!(extract_all_letters_spaced_by(&text, 4, 6), "eee");
    }

    /*#[test]
    fn test_we_can_find_most_probable_letters(){
        //          x     x     x     x
        let text = "ceciestletexteenclair";
        find_most_probable_key_letter(0, 6, &text, languages_letters_frequency::DICTIONARIES["fr"]);
    }*/
}
