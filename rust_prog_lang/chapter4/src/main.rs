fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1; // Move.
    //println!("{}, world!", s1); // Error, since s1 is no longer valid.

    let s1 = String::from("hello");
    let s2 = s1.clone(); // This is a copy.
    println!("s1={}, s2={}", s1, s2);

    let s= String::from("hello"); // s comes into scope.
    takes_ownership(s); // s values moves into the function.
    // s is no longer valid here.

    let x = 5;
    makes_copy(x); // uint types are copied.
    // Thus, x is still valid.
    println!("Other print {}", x); // This is valid.

    let s1 = gives_ownership();
    let s2 = String::from("hello world");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // References and borrowing.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable references.
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Scopes.
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world!");
    } // r1 goes out of scope here, a new reference can be established.

    let r2 = &mut s;
    r2.push_str(" Welcome to Rust!");
    println!("{}", r2);

    let s = String::from("Hello World!");
    println!("First word index {}", first_word(&s));

    // Slices.
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("First word {}", first_word2(&s));
    println!("First word {}", first_word3("Hello World!"));

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// some_string se elimina pues se tiene ownership.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
// Como no se tiene ownership de s, no se borra en este punto.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    return &s;
}
// Generates a dangling reference, s is deleted.
*/

// This is ok, ownership is mover out, no deallocation occurs.
fn no_dangle() -> String {
    let s = String::from("hello");
    return s;
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    return s.len();
}

// &str is a string slice
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}


fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}


