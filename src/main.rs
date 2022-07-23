fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 34, 5, 6, 3, 33, 11, 10, 7, 8, 5];
    let average = mean(&numbers);
    println!("Average is {}", average);
    //Mean of numbers
    let median = median(&numbers);
    println!("Median is {}", median);
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
    6
}
