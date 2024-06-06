fn main() {
    println!("Hello, world!");
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = "baz";
    s1.push_str(&s2);
    s1.push_str(s3);

    println!("{s1}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;


    //format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s = String::from("Привіт");

    for c in s.chars() {
        println!("{c}");
    }

    for b in s.bytes() {
        println!("{b}");
    }

    println!("{s}");
}
