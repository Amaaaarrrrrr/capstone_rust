use std::io;

fn main() {
    println!("Hello, Welcome To My Rust Tutorial!");

    // Ask for user's name
    let mut name = String::new();
    println!("What's your name?");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim(); // Remove newline

    // Ask for excitement level with validation loop
    let excitement_level: i32 = loop {
        let mut excitement_input = String::new();
        println!("On a scale of 1-10, how excited are you to learn Rust?");
        io::stdin().read_line(&mut excitement_input).expect("Failed to read input");

        match excitement_input.trim().parse::<i32>() {
            Ok(num) if (1..=10).contains(&num) => break num,
            _ => {
                println!("❌ Please enter a valid number between 1 and 10.");
                continue;
            }
        }
    };

    // Ask for year with validation loop
    let year: i32 = loop {
        let mut year_input = String::new();
        println!("Which year is it?");
        io::stdin().read_line(&mut year_input).expect("Failed to read input");

        match year_input.trim().parse::<i32>() {
            Ok(num) if num >= 1900 && num <= 2100 => break num,
            _ => {
                println!("❌ Please enter a valid year (e.g., 2024).");
                continue;
            }
        }
    };

    let language = "Rust";  

    println!("I'm learning {} in {}!", language, year);
    println!("My excitement level is: {}/10", excitement_level);

        // Show a custom message based on excitement level
    if excitement_level >= 8 {
        println!("🔥 You're super hyped! Let's dive deep into Rust!");
    } else if excitement_level >= 5 {
        println!("🙂 Nice! You're curious and ready to learn.");
    } else {
        println!("😅 Don’t worry, Rust will grow on you!");
    }

    greet_user(name);
}

fn greet_user(name: &str) {
    println!("Welcome to Rust, {}! 🦀", name);
    println!("Rust is memory-safe, fast, and fun to learn!");
}
