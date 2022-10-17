fn main(){
    let mut num = 5;
    let mut _increase = ||{
        num = num + 1;
    };

    _increase();
    println!("Num = {}", num);
    

    let _calc: fn(i8, i8) -> i8 = |a, b|{
        return a + b;
    };

    println!("Res 5 + 5 = {}", _calc(5, 5));
    
}