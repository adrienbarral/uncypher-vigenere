use crate::key_finder::AlphabetNormalized;

use std::collections::HashMap;
lazy_static! {
    static ref FRENCH_ALPHABET: AlphabetNormalized = {
        let m = AlphabetNormalized::from(vec![0.0942, 0.0102, 0.0264, 0.0339, 0.1587, 0.0095, 0.0104, 0.0077, 0.0841, 0.0089, 0.0, 0.0534, 0.0324, 0.0715, 0.0514, 0.0286, 0.0106, 0.0646, 0.0790, 0.0726, 0.0624, 0.0215, 0.0, 0.003, 0.0024, 0.0032]);
        m
    };
}

lazy_static! {
    static ref ENGLISH_ALPHABET: AlphabetNormalized = {
        let m = AlphabetNormalized::from(vec![0.0808, 0.0167, 0.0318, 0.0399, 0.1256, 0.0217, 0.0180, 0.0527, 0.0724, 0.0014, 0.0063, 0.0404, 0.0260, 0.0738, 0.0747, 0.0191, 0.0009, 0.0642, 0.0659, 0.0915, 0.0279, 0.01, 0.0189, 0.0021, 0.0165, 0.0007]);
        m
    };
}

lazy_static! {
    pub static ref DICTIONARIES: HashMap<&'static str, &'static AlphabetNormalized> = {
        let mut m: HashMap<&str, &AlphabetNormalized> = HashMap::new();
        m.insert("fr", &FRENCH_ALPHABET);
        m.insert("en", &ENGLISH_ALPHABET);
        m
    };
}
