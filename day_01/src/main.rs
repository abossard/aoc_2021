use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // split by linebreaks
    let lines: Vec<&str> = content.split("\n").collect();
    // iterate over lines
    let mut count = 0;
    let mut prev = 99999;
    for line in lines {
        // check line not empty
        if line.len() == 0 {
            continue;
        }
        // convert string to int
        let line = line.parse::<i32>().unwrap();
        if line > prev {
            count += 1;
        }
        prev = line;
        println!("{}", line);
    }  
    println!("{}", count);  
}
