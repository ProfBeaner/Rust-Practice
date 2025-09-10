use std::io; //imports input/output toolbox from standard library

fn frstfn() -> String { //returns function value as string
    println!("Bryce Snow, are you EXTREMELY OBESE? (y/n): "); //prints string literal into terminal 
    let mut choice = String::new(); //creates mutable string variable that is empty
    io::stdin() //imports standard input function from io toolbox 
        .read_line(&mut choice) //.read_line reads user input and stores it in "choice" variable
        .expect("Failed to read line"); //.expect returns error message if input fails
    choice.trim().to_lowercase() //trims whitespace & converts choice variable to lowercase
}

fn scndfn() { //second function
    loop { //loops function
        let choice = frstfn(); //calls first function and stores return value in choice variable 
        if choice == "y" { //if user input is "y"
            println!("Correct! :)"); //prints string literal into terminal
            break; //breaks loop
        } else { //if user input is not "y"
            println!("Incorrect. Try again. :("); //prints string literal into terminal 
        }
    }
}
pub fn totlfn1() { //makes necessary function public 
scndfn();
}