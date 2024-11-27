

pub fn count_word(message: &str, word: &str) -> u16 {
    let expected = word;
    let mut count = 0;
    let mut scanned_bytes = 0;
    // println!("string len: {}", message.len());
    while let Some(base_index) = message.get(scanned_bytes..).map(|m| m.find(|c: char| c == 'l' || c == 'L')).flatten() {
        let message = &message[scanned_bytes..];
        let mut i = 0;
        let mut message_index = base_index + i;
        // println!("scanning from {} {}", scanned_bytes + base_index, &message[message_index..message_index+1]);
        while let Some(s) = message.get(message_index..message_index + 1) {
            if i >= expected.len() {
                break;
            }
            let expected_now = &expected[i..i+1];
            // println!("hello {i}, {message_index}");
            if !matches_chars(expected_now, s) {
                break;
            }
            while let Some(s) = message.get(message_index..message_index + 1) {
                // println!("continuous thing: {message_index}");
                if matches_chars(expected_now, s) {
                    message_index += 1;
                } else {
                    break;
                }
            }
            i += 1;
        }
        if i >= expected.len() {
            count += 1;
        }
        let increment = message_index ;
        // println!("incremening {increment}");
        scanned_bytes += increment;
        // println!("scanned bytes: {scanned_bytes}");
    }
    return count;
}

fn matches_chars(expected: &str, got: &str) -> bool {
    got == expected || got.to_lowercase() == expected
}

#[cfg(test)]
mod tests {
    use crate::count_word;

    fn count_love(message: &str) -> u16 {
        count_word(message, "love")
    }

    #[test]
    fn test_double_l() {
        let s = "I'll miss you";
        assert_eq!(count_love(s), 0);
    }

    #[test]
    fn test2() {
        let s = "love it's easy to search but it's like tens of thousands messages probably";
        assert_eq!(count_love(s), 1);
    }

    #[test]
    fn case_change() {
        let s = "lOove it's easy LLloveee to search but it's like tens of thousands messages probably";
        assert_eq!(count_love(s), 2);
    }

    #[test]
    fn multiple_continuous() {
        let s = "lovelovelove";
        assert_eq!(count_love(s), 3);
    }

    #[test]
    fn single() {
        let s = "love";
        assert_eq!(count_love(s), 1);
    }
}
