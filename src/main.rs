use std::env;
use std::fs;
use std::collections::HashSet;


fn find_message_start(message: &str, marker_size: usize) -> Option<usize> {
    let starts = message
        .char_indices();
    for (i, _) in starts {
        if message.len() - 1 < i + marker_size {
            break;
        }
        let window = &message[i..i + marker_size];
        let char_set: HashSet<char> = HashSet::from_iter(window.chars());
        if char_set.len() == marker_size {
            return Some(i + marker_size);
        }
    }
    None
}


fn main() {
    let path = env::args().nth(1).expect("No file path!");

    let data = fs::read_to_string(path).expect("Could not open file!");

    match find_message_start(&data, 14) {
        Some(i) => println!("Message found at: {}", i),
        None => println!("No message found!")
    };
}


#[test]
fn test_simple() {
    assert_eq!(
        find_message_start("abcabcabcabcde", 4),
        Some(13)
    )
}

#[test]
fn test_not_there() {
    assert_eq!(
        find_message_start("abcabcabcabcabc", 4),
        None
    )
}

#[test]
fn test_empty() {
    assert_eq!(
        find_message_start("", 4),
        None
    )
}
