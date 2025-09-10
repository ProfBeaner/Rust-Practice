//Create a program that is a simple calculator that can compute statistical analysis on a dataset of numbers provided by user.

pub fn totlfn2() {
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust Stat Calculator!");
    println!("Enter a list of numbers separated by commas (e.g., 1,5,3,4,5).");
    println!("Type 'quit' to exit.");

    loop {
        // Read user input
        print!("\n> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("quit") {
            println!("Exiting program. Goodbye!");
            break;
        }

        // Parse numbers from input string
        let numbers: Vec<f64> = trimmed_input
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .collect();

        if numbers.is_empty() {
            println!("Invalid input. Please enter a list of numbers.");
            continue;
        }

        // Display results
        let mut sorted_numbers = numbers.clone();
        sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!("--- Statistics ---");
        println!("  Mean:   {}", mean(&numbers));
        println!("  Median: {}", median(&mut sorted_numbers));
        if let Some(mode) = mode(&numbers) {
            println!("  Mode:   {}", mode);
        } else {
            println!("  Mode:   No single mode found.");
        }
        println!("  Count:  {}", numbers.len());
        println!("------------------");
    }
}

/// Calculates the mean (average) of a slice of numbers.
fn mean(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

/// Calculates the median of a *sorted* slice of numbers.
fn median(numbers: &mut [f64]) -> f64 {
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
}

/// Calculates the mode of a slice of numbers.
/// Returns `None` if there is no single mode.
fn mode(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let mut occurrences: HashMap<f64, i32> = HashMap::new();
    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode_value = None;
    let mut multiple_modes = false;

    for (&value, &count) in occurrences.iter() {
        if count > max_count {
            max_count = count;
            mode_value = Some(value);
            multiple_modes = false;
        } else if count == max_count && max_count > 1 {
            multiple_modes = true;
        }
    }
    
    // Return None if no mode (all unique numbers) or multiple modes exist
    if multiple_modes || occurrences.len() == numbers.len() {
        None
    } else {
        mode_value
    }
}
}