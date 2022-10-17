#[derive(Clone, Copy)]
struct Rectangle{
    w: i32,
    h: i32
}

fn main(){
 
    {
        let _x = "rust".to_string();
    }

    // println!("{x}"); // Error

    let data = "hello world".to_string();
    //let new_data = data;// use .clone() for clonning
    println!("{data}"); // -> error

    let x = 1;
    let y = x;
    println!("x: {x} y: {y}");//Stack primitives

    let x = "hello world!".to_string();
    print(x);
    //println!("{x}");// heap owner error


    let xrec = Rectangle{
        w: 10,
        h: 10  
    };

    square(xrec);
    println!("{}", xrec.w);// Heap owner error if you run without line 1

}

fn print(m: String){
    println!("{m}");
}

fn square(rec: Rectangle) -> i32{
    return rec.w * rec.h;
}