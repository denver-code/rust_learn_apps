fn main(){
    let string = "World";
    println!("Word \"{string}\" is a palindrome -> {}", is_palindrome(string.to_string()))
}

fn reverse(string: String) -> String{
    return string.chars().rev().collect::<String>();
}

fn is_palindrome(word: String) -> bool{
    if word.to_lowercase() == reverse(word.to_string()).to_lowercase() {
        return true;
    }
    return false;
}