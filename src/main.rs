use std::io;

fn main () { 
    println!("Please input your temperature on faringate.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let f: f64 = input.trim().parse().unwrap();
    let c: f64 = ({f}-32.)*0.5556;
    println!("temperature is: {c}");
}