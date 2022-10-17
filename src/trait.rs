trait Employee{
    fn do_work(&self){
        println!("Do something like a employee!");
    }

    fn tale_salary(&mut self, salary: i32);
}

impl Employee for Human{
    fn do_work(&self){
        println!("Do something like a human!");
    }

    fn tale_salary(&mut self, salary: i32){
        self.salary += salary;
    }
}
impl ToString for Human{
    fn to_string(&self) -> std::string::String{
        format!("Hi my name is {}, am {} yo", self.name, self.age)
    }
}

struct Human{
    name: String,
    age: u8,
    salary: i32
}

impl Human{
    fn get_age(&self) -> u8{
        return self.age;
    }

    fn set_age(&mut self, new_age: u8) -> u8{
        self.age = new_age;
        return new_age;
    }

    fn new( name: String, age: u8, salary: i32) -> Human{
        return Human{
            name: name,
            age: age,
            salary: salary
        }
    }
}

fn main(){
    let mut igor = Human::new("Igor".to_string(), 17, 0);

    println!("Igor age -> {}", igor.get_age());
    igor.set_age(18);
    println!("Igor new age -> {}", igor.get_age());

    igor.do_work();
    igor.tale_salary(10);

    println!("igor: {}", igor.to_string());
}