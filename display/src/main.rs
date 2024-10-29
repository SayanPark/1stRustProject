#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to process.");
        }
        Message::Move { x, y } => {
            println!("Moving to coordinates: x = {}, y = {}", x, y);
        }
        Message::Write(text) => {
            println!("Writing message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB: ({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        process_message(msg);
    }
}