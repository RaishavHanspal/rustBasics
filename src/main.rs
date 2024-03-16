use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    start_scene();
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

    let mut min_age: u32 = 0;
    let mut max_age: u32 = 100;
    let mut trials: u32 = 0;
    const USE_MATCH: bool = false;
    const LOOP_EQ_FOR_WHILE: bool = true;
    // ideally we should use "loop"
    while LOOP_EQ_FOR_WHILE {
        let guess_age: u32 = rand::thread_rng().gen_range(min_age..=max_age);
        println!("[{host}:] Is it {guess_age}?");
        trials = trials + 1;
        let result = if USE_MATCH {
            age_guess_match((guess_age, age, name, host, trials, (min_age, max_age)))
        } else {
            age_guess_if_else(guess_age, age, (min_age, max_age))
        };
        if result.0 {
            break;
        }
        min_age = result.1 .0;
        max_age = result.1 .1;
    }
}

fn age_guess_match(data: (u32, u32, &str, &str, u32, (u32, u32))) -> (bool, (u32, u32)) {
    let guess_age: u32 = data.0;
    let age: u32 = data.1;
    let name: &str = data.2;
    let host: &str = data.3;
    let trials: u32 = data.4;
    let mut range: (u32, u32) = data.5;
    let mut _stop: bool = false;
    match guess_age.cmp(&age) {
        Ordering::Equal => {
            println!("[{name}:] Bingo...! It took you {trials} tries.");
            _stop = true;
        }
        Ordering::Greater => {
            println!("[{name}:] Do I look that old?");
            range.1 = guess_age - 1;
        }
        Ordering::Less => {
            println!("[{name}:] Haha.. I'm older than you think!");
            range.0 = guess_age + 1;
            match guess_age.cmp(&range.1) {
                Ordering::Equal => {
                    println!("[{host}:] Leave it, You must be very old then!");
                    _stop = true;
                }
                Ordering::Greater => {}
                Ordering::Less => {}
            }
        }
    }
    (_stop, range)
}

fn age_guess_if_else(guess_age: u32, age: u32, mut range: (u32, u32)) -> (bool, (u32, u32)) {
    let mut _stop: bool = false;
    if guess_age == age {
        println!("[You:] Bingo...! It took you some tries.");
        _stop = true;
    } else if guess_age > age {
        println!("[You:] Do I look that old?");
        range.1 = guess_age - 1;
    } else {
        println!("[You:] Haha.. I'm older than you think!");
        range.0 = guess_age - 1;
        if guess_age == range.1 {
            println!("[Host:] Leave it, You must be very old then!");
            _stop = true;
        }
    }
    return (_stop, range);
}

fn start_scene() {
    // loading
    println!("Scene starts in ");
    let abc = 1..6;
    for a in abc.rev() {
        println!("{a}");
    }
}
