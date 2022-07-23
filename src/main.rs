fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 34, 5, 6, 7, 8, 5];
    let average = mean(&numbers);
    println!("Average is {}", average);
    //Mean of numbers
}

fn mean(numbers: &[i32]) -> f64 {
    //Sum the number in the vector
    let mut sum: f64 = 0.0;
    for num in numbers {
        sum += *num as f64;
    }
    sum / numbers.len() as f64
    //Divide by the len of  the vector
}
fn median(numbers: Vec<i32>) -> i32 {
    9
}
fn mode(numbers: Vec<i32>) -> i32 {
    4
}
