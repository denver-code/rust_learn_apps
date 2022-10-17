struct Human{
    name: String,
    age: i8
}

fn main() {
    let igor = create_human("Igor Savenko".to_string(), 17);
    show_human(igor);
}

fn create_human(name: String, age: i8) -> Human{
    return Human{
        name: name.to_string(),
        age: age
    };
}

fn show_human(human: Human){
    let Human{name, age} = human;
    println!("Name: {name} Age: {age}" );
}

