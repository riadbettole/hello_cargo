struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word2(&s);
    let x = &mut s[0..5];
    println!("the first word is: {}", word);
    x.make_ascii_uppercase();

    println!("the first word is: {}", x);
}

fn return_first_word(text:&String)->String{
    let mut word = String::from("");
    for c in text.chars() {
        if c == ' '{
            break;
        }
        word.push(c)
    }
    word
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
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


