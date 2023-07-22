use std::io;

fn name_input() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();

    println!("Enter you first name: ");
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to read line");
    
    println!("Enter your last name: ");
    io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to read line");

    let full_name = format!("Your name is {} {}", first_name.trim(), last_name.trim());

    full_name
}

fn main() {
    let full_name = name_input();
    println!("{}", full_name)
}
