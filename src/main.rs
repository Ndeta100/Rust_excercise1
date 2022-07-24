use std::{collections::HashMap, io};

fn main() {
    println!("Hello, world!");
    let numbers = vec![5, 2, 34, 5, 6, 3, 5, 11, 10, 7, 8, 5];
    let average = mean(&numbers);
    println!("Average is {}", average);
    //Mean of numbers
    let median = median(&numbers);
    println!("Median is {}", median);
    mode(&numbers);
    let md = mode(&numbers);
    println!("Mode is {}", md);
    let word = "Ndeta";
    println!("{}", convert_to_pig_latin(word));
    directory();
}

fn mean(numbers: &[i32]) -> f64 {
    //Sum the number in the vector
    let mut sum: f64 = 0.0;
    for num in numbers {
        sum += *num as f64;
    }
    //Divide by the len of the vector
    sum / numbers.len() as f64
}
fn median(numbers: &[i32]) -> f64 {
    //Sort the vector
    //Get the middle number
    //If the vector has an event length we return the mean of the two middle numbers
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    println!("Sorted number in median {:?}", sorted_numbers);
    let middle = sorted_numbers.len() / 2;
    if sorted_numbers.len() % 2 == 0 {
        return mean(&vec![sorted_numbers[middle], sorted_numbers[middle - 1]]);
    }
    sorted_numbers[middle] as f64
}
fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    // let sum=numbers.iter().fold(0, |acc, curr|acc+curr);
    // sum as f64/numbers.len() as f64;
    println!("Map of occurances {:?}", map);
    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }
    mode
}
//You take the first letter of a word (e.g. Hello = H) and use the last letters (e.g. Hello = ello) and add 'ay' to the first letter (e.g. Hello = Ello hay). Words that start with a vowel (A, E, I, O, U) simply have "ay" appended to the end of the word
fn convert_to_pig_latin(word: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let (first, rest) = word.split_at(1);
    let is_vowel = vowels.contains(&first);
    if is_vowel {
        return format!("{}-{}", word, "hay");
    }
    format!("{}-{}ay", rest, first)
}

//Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn directory() {
    let mut employee_dir = HashMap::new();
    loop {
        println!("Enter a command like \"  Add <person> to <Department>\"");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read command");
        let command: &str = command.trim();
        let mut iter = command.split_whitespace();
        let person = iter.nth(1);
        let person = match person {
            Some(p) => p,
            None => {
                println!("Please enter a valid number");
                continue;
            }
        };
        let department = iter.nth(1);
        let department = match department {
            Some(p) => p,
            None => {
                println!("Please enter a valid number");
                continue;
            }
        };
        let employees = employee_dir
            .entry(String::from(department))
            .or_insert(vec![]);
        employees.push(String::from(person));
        println!("Employee directory: {:?}", employee_dir)
    }
}
