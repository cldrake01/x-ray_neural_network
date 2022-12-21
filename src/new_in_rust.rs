/*

use std::cmp::Ordering;
use std::io;
use hashbrown::HashMap;

struct Float(f64);

impl Eq for Float {}

impl PartialEq<Self> for Float {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}


fn main() {
    // prompt the user for how many values to collect
    println!("How many test scores would you like to enter? ");

    let mut num_scores = String::new();
    io::stdin().read_line(&mut num_scores).expect("Failed to read line");
    let num_scores: usize = num_scores.trim().parse().expect("Please enter a valid number");

    // create an array of the specified size
    let mut scores = Vec::new();

    // use a loop that "counts" from 0 through numScores-1,
    // prompts for doubles as test scores between 0.0 to 100.0,
    // and stores them in the array at an index based on the counter
    for i in 0..num_scores {
        println!("Enter test score {}: ", i + 1);
        let score = prompt_double(0.0, 100.0);
        scores.push(score);
    }

    // display the contents of the array to the console
    println!("Test scores: {:?}", scores);

    // call the helper methods to perform some analysis of the data in the array
    let min = min(&scores);
    let max = max(&scores);
    let mean = mean(&scores);
    let median = median(&scores);
    let mode = mode(&scores);

    // output the results in meaningful, well-formatted way
    println!("The maximum score is: {}", max);
    println!("The minimum score is: {}", min);
    println!("The mean is: {}", mean);
    println!("The median is: {}", median);
    println!("The mode is: {}", mode);
}

// helper method to prompt for and return a double value with error checking
fn prompt_double(low: f64, high: f64) -> f64 {
    loop {
        println!("Enter a value between {} and {}: ", low, high);

        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        let value: f64 = value.trim().parse().expect("Please enter a valid number");

        if value >= low && value <= high {
            return value;
        } else {
            println!("Error: Value must be between {} and {}", low, high);
        }
    }
}

// helper method to accept an array of doubles and return the smallest value
fn min(arr: &[f64]) -> f64 {
    // you may assume the array has at least 1 value
    // assume the first value is the minimum value
    let mut min = arr[0];

    // loop through the remainder of the array and find the minimum value
    for i in 1..arr.len() {
        if arr[i] < min {
            min = arr[i];
        }
    }

    // when the loop ends, return the minimum value
    min
}

// helper method to accept an array of doubles and return the largest value
fn max(arr: &[f64]) -> f64 {
    // you may assume the array has at least 1 value
    // assume the first value is the maximum value
    let mut max = arr[0];

    // loop through the remainder of the array and find the maximum value
    for i in 1..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }

    // when the loop ends, return the maximum value
    max
}

// helper method to accept an array of doubles and return the mean value
fn mean(arr: &[f64]) -> f64 {
    // you may assume the array has at least 1 value
    let mut sum = 0.0;

    // loop through the array and sum all the values
    for i in 0..arr.len() {
        sum += arr[i];
    }

    // when the loop ends, divide the sum by the number of values to find the mean
    sum / arr.len() as f64
}

// helper method to accept an array of doubles and return the median value
fn median(arr: &[f64]) -> f64 {
    // you may assume the array has at least 1 value
    // sort the array in ascending order
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // if the length of the array is odd, return the middle value
    if sorted_arr.len() % 2 == 1 {
        sorted_arr[sorted_arr.len() / 2]
    }
    // if the length of the array is even, return the mean of the two middle values
    else {
        (sorted_arr[sorted_arr.len() / 2 - 1] + sorted_arr[sorted_arr.len() / 2]) / 2.0
    }
}

// helper method to accept an array of doubles and return the mode value
fn mode(arr: &[f64]) -> f64 {
    // create a HashMap to store the count of each value
    let mut counts = HashMap::new();

    // loop through the array and update the count for each value
    for i in 0..arr.len() {
        let count = counts.entry(Float(arr[i])).or_insert(0);
        *count += 1;
    }
    // loop through the array and update the count for each value

    // find the maximum count in the HashMap
    let mut max_count = 0;
    for (_, count) in counts.iter() {
        if *count > max_count {
            max_count = *count;
        }
    }

    // if there is a tie for the maximum count, return the first value with the maximum count
    for (value, count) in counts.iter() {
        if *count == max_count {
            return value;
        }
    }

    // if no value has a count greater than 1, there is no mode
    0.0
}

 */