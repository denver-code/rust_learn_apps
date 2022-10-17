struct Human{
    name: String,
    age: u8
}

impl Human{
    fn get_age(&self) -> u8{
        return self.age;
    }

    fn set_age(&mut self, new_age: u8) -> u8{
        self.age = new_age;
        return new_age;
    }

    fn new( name: String, age: u8) -> Human{
        return Human{
            name: name,
            age: age
        }
    }
}

fn main(){
    let mut igor = Human::new("Igor".to_string(), 17);

    println!("Igor age -> {}", igor.get_age());
    igor.set_age(18);
    println!("Igor new age -> {}", igor.get_age());
}