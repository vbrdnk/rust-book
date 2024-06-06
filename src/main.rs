fn main() {
    println!("Hello, world!");

    let mut v = Vec::new();
    v.push(1);
    iterate_vector();


    // Using enums to represent the different types of data
    enum SpreadsheetCell {
        Int(i32),
        String(String),
        Float(f64),
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::String("Hello".to_string()),
        SpreadsheetCell::Float(3.14),
    ];
}

fn iterate_vector() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    println!("{:?}", v);
}

fn read_vector_value() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("The third element is not present"),
    }
}
