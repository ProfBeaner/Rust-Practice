use std::io;

fn main_function() -> String {
    println!("Bryce Snow, are you EXTREMELY OBESE? (y/n): ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice.trim().to_lowercase()
}

fn main() {
    // Loop infinitely until a 'break' statement is executed.
    loop {
        let choice = main_function();
        if choice == "y" {
            println!("Correct! :)");
            break; // Exit the loop when the answer is correct.
        } else {
            println!("Incorrect. Try again. :(");
        }
    }
}

