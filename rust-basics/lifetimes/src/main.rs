fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("rfc");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
