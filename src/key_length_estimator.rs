use std::collections::HashMap;
use std::fmt;

type RepeatedSequences = HashMap<String, Vec<usize>>;

/// Detect key length.
pub fn detect_key_length(text: &str, max_key_length: usize) -> usize {
    let repeated_sequences = fill_map_of_repeated_sequences(text, 2);
    print_map(&repeated_sequences);
    let distances_histogram =
        get_distances_histogram_of_repeated_sequences(&repeated_sequences, max_key_length);
    println!("distances : {:?}", distances_histogram);
    return get_max_position_in_vector(&distances_histogram);
}

fn get_max_position_in_vector(vector: &Vec<usize>) -> usize {
    let mut max = 0;
    let mut res = 0;
    for pos in 0..vector.len() {
        if max < vector[pos] {
            max = vector[pos];
            res = pos;
        }
    }
    return res;
}
/// A distance histogram will be a vector of size `max_distance`.
/// `res[1]` will be the number of occurance of the distance 1 in repeated sequences. As an exemple in sentence :
/// 'abcdefabjkabjoef' repeatitions of 'ab' are at distance 5 and 2. repetitions of "ef" is at distance 8.
/// So res[2] = res[5] = res[8] = 1, other fields of res ar null.
fn get_distances_histogram_of_repeated_sequences(
    sequences: &RepeatedSequences,
    max_distance: usize,
) -> Vec<usize> {
    let mut res = Vec::new();
    res.resize_with(max_distance+1, || 0);

    for (_, positions) in sequences {
        for i in 0..positions.len() - 1 {
            let distance = positions[i + 1] - positions[i];
            if distance < max_distance {
                res[distance] = res[distance] + 1;
            }
        }
    }
    return res;
}

/// Construct and fill a map of repeated sequences in text passed in argument
/// Key of map will be the sequence, and value of this map will be a vector of the positions
/// in the text.
/// e.g. : 'abcdefabjkabjoef' will return the following map for a `seq_length = 2`:
/// ```json
/// {
///    'ab': [0, 6, 10],
///    'ef': [4,14]
/// }
/// ```
fn fill_map_of_repeated_sequences(text: &str, seq_length: usize) -> RepeatedSequences {
    let mut res: RepeatedSequences = HashMap::new();

    // First we fill our map with all sequences possible (and their repetitions if any)
    let mut pos = 0;
    while pos + seq_length <= text.len() {
        let seq = &text[pos..(pos + seq_length)];
        match res.get_mut(seq) {
            Some(positions) => {
                positions.push(pos);
            }
            None => {
                res.insert(String::from(seq), vec![pos]);
            }
        }
        pos = pos + 1;
    }

    // Then we clean our map by removing all non repeating sequences, i.e. all sequences with only 1 position.
    remove_non_repeating_sequences(&mut res);
    return res;
}

fn remove_non_repeating_sequences(map: &mut RepeatedSequences) {
    map.retain(|_, repeats| repeats.len() > 1);
}

fn print_map(map: &RepeatedSequences) {
    for (key, value) in map {
        println!("{}:", key);
        for values in value {
            print!("{},", values);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_we_can_remove_non_repeating_sequences() {
        let mut res: RepeatedSequences = HashMap::new();
        res.insert(String::from("toto"), vec![0]);
        res.insert(String::from("tata"), vec![0, 1]);
        assert_eq!(res.len(), 2);
        remove_non_repeating_sequences(&mut res);
        assert_eq!(res.len(), 1);
        assert_eq!(res.contains_key(&String::from("tata")), true);
    }

    #[test]
    fn test_we_can_detect_repeated_sequence() {
        let input = "abcdefabjkabjoef";
        let res = fill_map_of_repeated_sequences(&input, 2);
        print_map(&res);
        assert_eq!(res.len(), 3); // "ab" and "ef" and "bj"

        match res.get("ab") {
            Some(positions) => {
                assert_eq!(positions.len(), 3);
                assert_eq!(positions[0], 0);
                assert_eq!(positions[1], 6);
                assert_eq!(positions[2], 10);
            }
            None => {
                panic!("Can't find \"ab\" in repeated sequences !");
            }
        }

        assert_eq!(res.get("ef").is_some(), true);
    }

    #[test]
    fn test_we_can_get_distances_from_repeated_sequences() {
        let mut res: RepeatedSequences = HashMap::new();
        res.insert(String::from("toto"), vec![0, 5, 7, 12, 17]);
        res.insert(String::from("tata"), vec![0, 2, 3, 8, 13, 14]);
        let distances = get_distances_histogram_of_repeated_sequences(&res, 10);
        assert_eq!(distances[5], 5);
        assert_eq!(distances[2], 2);
        assert_eq!(distances[1], 2);
    }
}
