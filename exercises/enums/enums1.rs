// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move{ x:i32,y:i32 },
    ChangeColor(i32,i32,i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move{x:2,y:3});
    println!("{:?}", Message::ChangeColor(255,255,0));
}
