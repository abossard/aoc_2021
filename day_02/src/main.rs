use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "input.txt";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // split by linebreaks
    let lines = content.split("\n");
    // convert itertor to vector
    let nums:Vec<(&str, i32)> = lines
        .filter(|s| s.len() > 0)
        .map(|s| s.split(" ").collect())
        // map tuple
        .map(|s:Vec<&str>| (s[0], s[1].parse::<i32>().unwrap()))
        .collect();//.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    // enumerate lines
    // iterate over lines
    let mut x = 0;
    let mut d = 0;
    let mut aim = 0;
    for (command, value) in nums {
        match command {
            "forward" => {x += value; d += aim * value},
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("ups")
        }
    }
    
    println!("{}", d * x);
    
}
