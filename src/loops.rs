fn main(){
    // Infinity loop
    println!("Infinity loop has been started.");
    let mut _f = 0;
    loop{
        println!("F: {_f}");
        if _f >= 10{
            break;
        }
        _f += 1;
    }
    println!("Infinity loop has been ended by break;.");

    
    // While loop
    let mut _j = 0;
    println!("While loop has been started.");
    while _j <=  10{
        println!("J: {_j}");
        _j += 1;
    }
    println!("While loop has been ended.");


    // For loop
    println!("For loop has been started.");
    for _i in 0..=10{
        println!("I: {_i}")
    }
    println!("For loop has been ended.");
}