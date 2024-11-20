use std::io;

fn main() {
    println!("Input a sentence:");
    
    let mut test_string = String::new();
    
    io::stdin().read_line(&mut test_string)
    .expect("Failed to read");
    
    let word = first_word_finder(&test_string);

    println!("{}", word);
}

fn first_word_finder(s:&String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
