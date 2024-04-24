use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // vector
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];

    let third: &i32 = &v1[2];
    let thr: Option<&i32> = v1.get(2);

    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);

    println!("{:?}", v);
    println!("{:?}", v1);

    println!("{third}");

    match thr {
        Some(thr) => println!("The third value is {thr}"),
        None => println!("There is no third value"),
    }

    // iterating over the vector
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // String
    let mut s = String::new();
    s = String::from("Hello, World");

    println!("{:?}", s);

    s.push_str("tarun");

    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from(" Tarun");
    let s3: String = s1 + &s2;

    let hello = String::from("नमस्ते");

    // printing in bytes
    for b in "नमस्ते".bytes() {
        print!("{b}, ")
    }
    println!();

    // printing all the scalar values
    for c in "नमस्ते".chars() {
        print!("{c}, ")
    }
    println!();

    // printing all the graphene
    for g in "नमस्ते".graphemes(true) {
        print!("{g}, ")
    }
    println!();

    // println!("{s3}");

    // Hashmaps
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    println!("{:#?}", scores);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    match score {
        Some(score) => println!("{score}"),
        None => println!("No value"),
    }

    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Blue"), 200);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    println!("{:#?}", scores);
}
