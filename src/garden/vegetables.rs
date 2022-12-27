pub trait SayHello {
    fn say_hello(&self) -> String;
}

#[derive(Debug)]
pub struct Asparagus {
    pub name: String,
    pub age: u8,
    pub is_green: bool,
    is_delicius: bool
}

impl Asparagus {
    pub fn new(name: &String, age: u8, is_green: bool, is_delicius: bool)-> Asparagus {
        Asparagus { name: name.into(), age: age, is_green: is_green, is_delicius: is_delicius }
    }
}

impl SayHello for Asparagus {
    fn say_hello(&self) -> String {
        let mut outcome = format!("Hello! i'm {}, i'm {} years old and ", self.name, self.age);

        if self.is_green {
            outcome.push_str("i'm green");
        } else {
            outcome.push_str("i'm not green");
        }

        if self.is_delicius {
            outcome.push_str(" great!!");
        } else {
            outcome.push_str(" dui!!");
        }

        outcome
    }
}