use std::io;

fn frstfn() -> String { //first function (to be looped)
    println!("Bryce Snow, are you EXTREMELY OBESE? (y/n): ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice.trim().to_lowercase()
}

fn scndfn() { //second function (creates loop)
    loop {
        let choice = frstfn();
        if choice == "y" {
            println!("Correct! :)");
            break;
        } else {
            println!("Incorrect. Try again. :(");
        }
    }
}
pub fn totlfn1() { //total function 
        scndfn();
}