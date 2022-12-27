use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let number1: i32;
    let number2: i32;
    let mut input_text: String;
    let mut another_text: String;

    println!("Hello, world!");

    //Works because is the first assginment
    number1 = 25;
    number2 = 34;

    let mut result = add(number1, number2);
    println!("add(25, 34): {}", result);

    //Works because number1 and number2 are primitive types
    result = add(number1, number2);
    println!("add(10, 45): {}", result);

    input_text = String::from("hello input Text");
    println!("{}", input_text);

    input_text.push_str(", concat!");

    println!("{}", input_text);

    //This works because concat_strings picks params borrowing them (with &)
    another_text = String::from("anotherText");
    input_text = concat_strings(&input_text, &another_text);

    println!("{}", input_text);

    //This works because input_text is reassigned with the result of concat_strings.
    //Otherwhise it would be borrowing error
    another_text = String::from("another text");
    input_text = concat_strings_without_borrowing(input_text, another_text);

    println!("{}", input_text);

    //This works because calculate_length picks param borrowing it (with &)
    //OtherWhise next println!("{}", input_text); will have an error
    let length = calculate_length(&input_text);
    println!("length: {}", length);

    println!("{}", input_text);

    //Example importing module
    print_vegetable();
}


fn add(number1: i32, number2: i32 )-> i32 {
    //We can write return number1 + number2; 
    //or this way, without return, without semicolon and being the last row of function
    number1 + number2
}

fn concat_strings_without_borrowing(string1: String, string2: String) -> String {
    format!("{},{},{}", string1, string2, string1.len()) 
}

fn concat_strings(string1: &String, string2: &String) -> String {
    format!("{},{},{}", string1, string2, string1.len()) 
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn print_vegetable() {
    let plant = Asparagus {name: "my asparagus".to_string(), age: 2, is_green: true};
    println!("I'm growing {:?}!", plant); //{:?} means print "plant" as string on debug mode. Otherwhise is not printable
}