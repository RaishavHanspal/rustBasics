use rand::Rng;
use std::io;
fn main() {
    // using constants we cannot reassign the value, or redeclare
    const TIME: u32 = 1230;
    // by default we cannot reassign the value, but we can redeclare same name variable
    let name = "Mario";
    // only using mut property we can reassign same variable
    let mut age = 26;
    // string literal '{}' is a must when printing variables/constants
    println!("Time: {}", TIME);
    println!("Hola! Yo Soy {}, I'm {} years old.", name, age);
    let mario = name;
    let name = "Luigi";
    age = 30;
    // variables and constants can only be accesible in their defined scope
    {
        let name = "Bowser";
        let age = age + 4;
        println!("...Meanwhile, {name} ({age} years old), looking from a distance!")
    }
    println!("Mucho Gusto {mario}, Yo Soy {name}, I'm {age} years old.");
    let mut name = String::new();
    let mut age = String::new();
    println!("What's your name?");
    io::stdin().read_line(&mut name).expect("name");
    println!("Hi {} How old are you?", name);
    // if we don't add use std::io at the top, we can directly access the same functionality prefixing std:: as used below
    std::io::stdin().read_line(&mut age).expect("age");
    let age = rand::thread_rng().gen_range(20..=50);
    println!("Okay! I thought it would be {age}.");
}
