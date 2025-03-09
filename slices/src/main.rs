fn main() {
    let string_literal = "literal"; // This is &str, thus making it immutable
    let mut first_test = String::from("uma string");
    let word = first_word(&first_test);

    // Problem is that word is not true anymore at after this
    first_test.clear();

    let s = String::from("Hello word");

    let hello = &[0..5];
    let world = &[6..11];
    
    let sliced = first_word_with_slices(&s);

    let arr = [5,4,3,2,1,0];
    let arr_slice = &arr[1..3];
    assert_eq!(arr_slice, [4,3])
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &String would only accept only String
// &str accepts literals
fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return s
}