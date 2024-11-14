

// Given a list of integers, use a vector and return 
// the median (when sorted, the value in the middle position) and mode 
// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn average(numbers: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for num in numbers {
        sum += num
    }
    sum
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    sorted[sorted.len() / 2]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    let copy: Vec<i32> = numbers.clone();
    let mut counts: HashMap<i32, i32>  = HashMap::new();
    let mut biggest: (i32, i32) = (0, 0);

    for num in copy {
        let count = counts.entry(num).or_insert(0);
        *count += 1;

        if count > &mut biggest.1 {
            biggest = (num, *count);
        }
    }

    biggest.0
}

fn main() {
    let numbers = vec![10,4,5,20,4,5,3,10,1];

    println!("{}", average(&numbers));
    println!("{}", median(&numbers));
    println!("{}", mode(&numbers));
    println!("{:?}", &numbers);

}
