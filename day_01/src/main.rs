use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // split by linebreaks
    let lines = content.split("\n");
    // convert itertor to vector
    let nums = lines.filter(|s| s.len() > 0).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // enumerate lines
    // iterate over lines
    let mut count = 0;
    let mut prev = 99999;
    // convert string vector to int vector
    // enumerate nums
    for (i, num) in nums.iter().enumerate() {
        // check line not empty
        if i + 2 == nums.len() {
            break;
        }

        // convert string to int
        // sum list of ints
        let sum = num + nums[i + 1] + nums[i + 2];
        if sum > prev {
            count += 1;
        }
        prev = sum;
        println!("{}", sum);
    }  
    println!("{}", count);  
}
