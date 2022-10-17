struct Rectangle(i8, i8);

fn main(){
    let mut tuple = ("Igor", 17, "Rust");
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);

    tuple.0 = "Denver";
    println!("{}", tuple.0);

    let (name, age, lang) = tuple;
    println!("{name}, {age}, {lang}");

    let x = Rectangle(15, 5);
    println!("Square of rectangle -> {}cm", rectangle_area(x));
}

fn rectangle_area(rectangle: Rectangle) -> i8{
    return rectangle.0 * rectangle.1;
}