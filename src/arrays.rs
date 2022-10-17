fn main(){
    let mut usernames = ["Igor", "Denver", "DSesh"];
    println!("First username is -> {}", usernames[0]);

    usernames[0] = "Savenko";
    println!("First username is -> {}", usernames[0]);

    for _i in usernames.iter(){
        println!("{_i}");
    }

    let _numbers: [i8; 5] = [1; 5];
    println!("{:?}", _numbers);

}