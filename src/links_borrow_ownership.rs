fn main(){
    let s1 = "hello world".to_string();
    take_ownership(s1);
    //println!("{s1}")// Error

    let mut s2 = "hello world2".to_string();
    s2 = borrow_ownership(s2);
    println!("s2: {s2}");

    let s3  = "hello world3".to_string();
    let s3_ref = &s3;
    print_linked(s3_ref);
    println!("s3: {s3}");

    let mut s4 = "hello world4".to_string();
    let s4_ref = &mut s4;
    modify_linked(s4_ref);
    println!("s4: {s4}");

    let mut s5 = "hello wolrd5".to_string();
    let s5_tuple = borrow_ownership_with_logic(s5);
    s5 = s5_tuple.1;
    println!("s5: {s5} result: {}", s5_tuple.0);
} 

fn take_ownership(s: String){
    println!("TW: {s}");
}

fn borrow_ownership(s: String) -> String{
    println!("BW: {s}");
    return s;
}

fn borrow_ownership_with_logic(s: String) -> (i8, String){
    println!("BOWL: {s}");
    return (5 + 5, s);
}

fn print_linked(s: &String){
    println!("PL: {s}");
}

fn modify_linked(s: &mut String){
    s.push_str(" pingpong");
    println!("ML: {s}");
}