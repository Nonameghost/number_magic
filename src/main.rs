use std::io;
use rand::Rng;

fn main() {
    let mut input = String::new();

    println!(">> Welcome to Number Magic!");

    loop{
        print!(">> ");
    
        match io::stdin().read_line(&mut input){
            Ok(_n) => match parse_input(&mut input){
                false => break,
                _ => {}
            },
            Err(e) => println!("Error: {}", e)
        }

        input.clear();
    }
    
}

fn parse_input(to_parse : &String) -> bool{
    let tokens = to_parse.split_whitespace().collect::<Vec<&str>>();

    if tokens.len() > 0 {
        match tokens[0] {
            "add" => add(&tokens),
            "sub" | "subtract" => subtract(&tokens),
            "mul" | "multiply" => multiply(&tokens),
            "div" | "divide" => divide(&tokens),
            "rand" | "random" => random_number(&tokens),
            "fact" | "factorial" => factorial(&tokens),
            "exit" | "quit" => return false,
            _ => println!("{} is not a valid command.", tokens[0])
        }
    }

    return true;
   
}

fn add(tokens : &Vec<&str>){
    if tokens.len() < 2 {
        println!("Pass arguments to add.");
        return;
    }

    let mut acc : i32 = 0;
    for i in 1 .. tokens.len(){
        match tokens[i].parse::<i32>(){
            Ok(n) => { acc += n },
            Err(e) => { 
                println!("Argument #{}: '{}' is not a valid number. Error: {}", i-1, tokens[i], e);
                return;
            }
        }
    }

    println!("Result: {}", acc);
}

fn subtract(tokens : &Vec<&str>){
    if tokens.len() < 2 {
        println!("Pass arguments to subtract.");
        return;
    }

    let mut acc : i32 = 0;
    for i in 1 .. tokens.len(){
        match tokens[i].parse::<i32>(){
            Ok(n) => { 
                match i {
                    1 => acc = n,
                    _ => acc -= n
                }
            },
            Err(e) => { 
                println!("Argument #{}: '{}' is not a valid number. Error: {}", i-1, tokens[i], e);
                return;
            }
        }
    }

    println!("Total: {}", acc);
}

fn multiply(tokens : &Vec<&str>){
    if tokens.len() < 2 {
        println!("Pass arguments to multiply.");
        return;
    }

    let mut acc : i32 = 1;
    for i in 1 .. tokens.len(){
        match tokens[i].parse::<i32>(){
            Ok(n) => acc *= n ,
            Err(e) => { 
                println!("Argument #{}: '{}' is not a valid number. Error: {}", i-1, tokens[i], e);
                return;
            }
        }
    }

    println!("Result: {}", acc);
}

fn divide(tokens : &Vec<&str>){
    if tokens.len() < 2 {
        println!("Pass arguments to multiply.");
        return;
    }

    let mut acc : i32 = 0;
    for i in 1 .. tokens.len(){
        match tokens[i].parse::<i32>(){
            Ok(n) => {
                match i {
                    1 => acc = n, 
                    _ => match n {
                        0 => {
                            println!("Argument #{} is zero. Cannot divide by zero.", i-1);
                            return;
                        },
                        _ => acc /= n
                    }
                }
            },
            Err(e) => { 
                println!("Argument #{}: '{}' is not a valid number. Error: {}", i-1, tokens[i], e);
                return;
            }
        }
    }

    println!("Result: {}", acc);
}

fn random_number(tokens : &Vec<&str>){
    if tokens.len() < 2 {
        println!("Pass arguments to get a random number.");
        return;
    }

    if tokens.len() > 3 {
        println!("Too many arguments, 'rand' expects 2 arguments.");
        return;
    }
    
    let first_bound = match parse_int(tokens[1]){
        None => {
            println!("Argument #1 '{}' is invalid", tokens[1]);
            return;
        },
        Some(n) => n
    };

    let mut rng = rand::thread_rng();

    if tokens.len() == 2
    {
        println!("Result: {}",  rng.gen_range(0, first_bound));
        return;
    }

    let second_bound = match parse_int(tokens[2]){
        None => {
            println!("Argument #2 '{}' is invalid", tokens[2]);
            return;
        },
        Some(n) => n
    };

    println!("Result: {}",  rng.gen_range(first_bound, second_bound));

}

fn factorial(tokens : &Vec<&str>){

    if tokens.len() != 2 {
        println!("Expects a single positive integer argument of which the factorial will be calculated.");
        return;
    }

    let num = match parse_int(tokens[1]) {
        Some(n) => n,
        _ => {
            println!("'{}' is not a valid number", tokens[1]);
            return;
        }
    };

    if num < 0 {
        println!("Does not support negative numbers because I'm a dumbass.");
        return;
    }

    //Cheating?
    if num == 0 {
        println!("Result: 1");
        return;
    }

    let mut i = num;
    let mut acc = 1;

    while i > 0 {
        acc *= i;
        i -= 1;
    }

    println!("Result: {}", acc);
}

fn parse_int(to_parse : &str) -> Option<i32>{
    match to_parse.parse::<i32>(){
        Ok(n) => Some(n),
        Err(_e) => None
    }
}
