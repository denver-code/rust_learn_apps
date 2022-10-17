fn main(){
    print_hello_world();
    println!("{}", welcome_message(1));
    println!("{}", welcome_message(2));
    println!("{}", welcome_message(5));
}

fn print_hello_world(){
    println!("Hardcode Hello world!");
}

fn welcome_message(_type: i8) -> String{
    match _type{
        1 => "Hello World!".to_string(),
        2 => "Hello From Rustlang!".to_string(),
        _ => "Hello everybody!".to_string()
    }
}