struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let mut user2 = User {
        ..user1
    };
    user2.email = String::from("someone@example.com");
    println!("{}",user2.email);
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

fn first_word2(s: &mut str) -> &mut str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &mut s[0..i];
        }
    }
    &mut s[..]
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


