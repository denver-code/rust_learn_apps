enum Operation{
    Add,
    Multiply
}

enum Cmd{
    Print(String),
    ChangeColor(Color)
}

struct Color{
    r: i32,
    g:  i32,
    b: i32
}

fn main(){
    let op = Operation::Add;
    println!("Result 3 + 5 = {}", calc(op, 3, 5));

    let cmd = Cmd::Print("Enum test".to_string());


    match cmd{
        Cmd::Print(message) => println!("message: {message}"),
        Cmd::ChangeColor(color) => println!("r:{} g:{} b:{}", color.r, color.g, color.b),
        _ => println!("Cmd _")
    }

    let mut x : Option<String> = Option::Some("Rust".to_string());
    x = Option::None;

    let data = x.unwrap();
    println!("{data}");
}

fn calc(op: Operation, x: i32, y: i32) -> i32{
    match op{
        Operation::Add => x+y,
        Operation::Multiply => x*y,
        _ => 0
    }
}