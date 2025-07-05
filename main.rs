use std::io;
use regex::Regex;

fn main() {
    let operation = read_input();
    println!("{:?}", calculate(operation));
}
// Function to read input from the user
// todo: add exit
fn read_input() -> String {
    println!("enter your problem:");
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.to_string();
        }
        Err(_) => {
            println!("There was an error reading your input. Please try again.");
            return read_input();
        }
    }
}

fn calculate(input: String) -> Vec<String> {
    // split the input using regex into tokens
    let re: Regex = Regex::new(r"(\d+|\+|\-|\*|\/|\^|\(|\))").unwrap();
    let mut tokens = re
        .find_iter(&input)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();

    // generate a reverse polish notaion using the shunting yard algorithm
    let mut output = Vec::new();
    let mut stack = Vec::new();

    // iterate through the tokens
    // if the token is a number, add it to the output
    while tokens.len() > 0 {
        if tokens[0].parse::<f64>().is_ok() {
            output.push(tokens[0].clone());
            tokens.remove(0);
        } else if
            // if the token is an operator:
            // if it is an exponentiation operator, push it directly to the stack since it is of highest priority
            tokens[0] == "^"
        {
            stack.push(tokens[0].clone());
            tokens.remove(0);
        } else if
            //if it is a multiplication or division:
            //check for if there are elemnts of higher priority in the stack (^)
            //pop them to the output then add the current token to the stack
            tokens[0] == "*" ||
            tokens[0] == "/"
        {
            while
                !stack.is_empty() &&
                (stack[stack.len() - 1] == "^" ||
                    stack[stack.len() - 1] == "*" ||
                    stack[stack.len() - 1] == "/")
            {
                output.push(stack.pop().unwrap());
            }
            stack.push(tokens[0].clone());
            tokens.remove(0);
        } else if
            //if it is a addition or subtraction:
            //check for if there are elemnts of higher priority in the stack (* or /)
            //pop them to the output then add the current token to the stack
            tokens[0] == "+" ||
            tokens[0] == "-"
        {
            while
                !stack.is_empty() &&
                (stack[stack.len() - 1] == "^" ||
                    stack[stack.len() - 1] == "*" ||
                    stack[stack.len() - 1] == "/" ||
                    stack[stack.len() - 1] == "+" ||
                    stack[stack.len() - 1] == "-")
            {
                output.push(stack.pop().unwrap());
            }
            stack.push(tokens[0].clone());
            tokens.remove(0);
        } else if tokens[0] == "(" {
            stack.push(tokens[0].clone());
            tokens.remove(0);
        } else if tokens[0] == ")" {
            while !stack.is_empty() && stack[stack.len() - 1] != "(" {
                output.push(stack.pop().unwrap());
            }
            stack.pop().unwrap();
            tokens.remove(0);
        }
    }
    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }
    return output;
}
