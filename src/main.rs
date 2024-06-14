fn main() {
    println!("Hello, world!");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    {
        let string3 = "xyze";
        let result = longest(string1.as_str(), string3);
        println!("The longest string is: {}", result);
    }

    println!("The longest string is: {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
