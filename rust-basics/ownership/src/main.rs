fn main() {
    // at any given time we can only have one mutable reference and any number unmutable reference
    // references must always be valid
    let mut s = String::from("hello world");
    take_ownership(&mut s);
    // println!("{s} has the address {:?}", &s);

    // let s2 = gives_ownership(s);
    // println!("value of s2 is {s2}");

    // slices
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn take_ownership(s: &mut String) {
    s.push_str("World");
    println!("{s} is the string");
}

fn gives_ownership(s: String) -> String {
    s
}
