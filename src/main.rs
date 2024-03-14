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
        println!(
            "...Meanwhile, {} ({} years old), looking from a distance!",
            name, age
        )
    }
    println!(
        "Mucho Gusto {}, Yo Soy {}, I'm {} years old.",
        mario, name, age
    );
}
