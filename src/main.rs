struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");


    // Matching ranges of values using `..=`
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let z = 'c';

    match z {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to break apart values

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant had no data to destructure"),
        Message::Move { x, y } => println!("Moved in the x direction {x} and in the y direction {y}"),
        Message::Write(s) => println!("Text message: {s}"),
        Message::ChangeColor(r, g, b) => println!("Set color to ({r}, {g}, {b})"),
    }


    // Match guardsw

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => println!(),
    }

    // @ Bindings

    enum TelegramMessage {
        Hello { id: i32 }
    }

    let msg = TelegramMessage::Hello { id: 5 };

    match msg {
        TelegramMessage::Hello { id: id_variable @ 3..=7 } => println!("Found an id in range: {id_variable}"),
        TelegramMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        TelegramMessage::Hello { id } => println!("Found some other id: {id}"),
    }
}
