use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    prologue();
}

fn prologue() {
    // using constants we cannot reassign the value, or redeclare
    const TIME: u32 = 1230;
    // by default we cannot reassign the value, but we can redeclare same name variable
    let name = "Mario";
    // only using mut property we can reassign same variable
    let mut age = 26;
    // string literal '{}' is a must when printing variables/constants
    println!("Time: {}", TIME);
    println!("[{}:] Hola! Yo Soy {}, I'm {} years old.", name, name, age);
    let mario = name;
    let name = "Luigi";
    age = 30;
    // variables and constants can only be accesible in their defined scope
    {
        let name = "Bowser";
        let age = age + 4;
        println!("...Meanwhile, {name} ({age} years old), looking from a distance!")
    }
    println!("[{name}:] Mucho Gusto {mario}, Yo Soy {name}, I'm {age} years old.");
    your_info(mario);
}

fn your_info(host: &str) {
    let mut name: String = String::new();
    let mut age = String::new();
    println!("[{host}:] What's your name?");
    io::stdin().read_line(&mut name).expect("name");
    let name = name.trim();
    println!("[{host}:] Hi {}, How old are you?", name);
    // if we don't add use std::io at the top, we can directly access the same functionality prefixing std:: as used below
    std::io::stdin().read_line(&mut age).expect("age");
    let age: u32 = age.trim().parse().expect("Please type a number!");
    println!("...Your silence, makes {host} nervous");

    let mut min_age = 0;
    let mut max_age = 100;
    let mut trials: u32 = 0;
    loop {
        let guess_age = rand::thread_rng().gen_range(min_age..=max_age);
        println!("[{host}:] Is it {guess_age}?");
        trials = trials + 1;
        match guess_age.cmp(&age) {
            Ordering::Equal => {
                println!("[{name}:] Bingo...! It took you {trials} tries.");
                break;
            }
            Ordering::Greater => {
                println!("[{name}:] Do I look that old?");
                max_age = guess_age - 1;
            }
            Ordering::Less => {
                println!("[{name}:] Haha.. I'm older than you think!");
                min_age = guess_age + 1;
            }
        }
    }
}
