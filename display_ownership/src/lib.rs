pub fn read_only() -> () {
    fn move_string(string: String) {
        println!("Moving string with content: {}", string)
    }

    fn borrow_string(string: &str) {
        println!("Borrowing string with content: {}", string)
    }

    println!("Ownership readonly:");

    // let move_string = |string: String| { println!("Moving string with content: {}", string) };
    let moved = String::from("String to be moved");
    (|| { println!("Cloning string with content: {}", moved.clone()) })();
    move_string(moved);
    // move_string(moved); // Is invalid

    let move_string_literal = |string_literal: &str| {
        println!("Moving string literal with content: {}", string_literal);
    };
    let to_be_copied = "String to be copied";
    move_string_literal(to_be_copied);
    println!("Borrowing string literal second time with content: {}", to_be_copied);
    (|string_literal: &str| {
        println!("Borrowing string literal third time with content {}", string_literal);
    })(to_be_copied);

    let give_ownership = || { String::from("ownership over string was given") };
    let owned_string = give_ownership();
    move_string(owned_string);
    // println!("Owning string: {}", owned_string); // Is invalid

    let owned_string = give_ownership();
    borrow_string(&owned_string);
    println!("Still owning string: {}", &owned_string);
    move_string(owned_string);
    //println!("Owning string: {}", owned_string); // Is invalid
    println!();
}

pub fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    string
}

#[cfg(test)]
mod first_word_tests {
    #[test]
    fn given_string() {
        let string = String::from("First Second Third");
        let first_word = super::first_word(&string);
        assert_eq!(first_word, "First");
    }
    #[test]
    fn given_empty_string() {
        let string = String::from("");
        let first_word = super::first_word(&string);
        assert_eq!(first_word, "");
    }
    #[test]
    fn given_string_without_spaces() {
        let string = String::from("MySuperWordWithManyWordsTogether");
        let first_word = super::first_word(&string);
        assert_eq!(first_word, "MySuperWordWithManyWordsTogether");
    }
}