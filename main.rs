use std::io;
use regex::Regex;

fn main() {
    let operation = read_input();
    println!("{:?}", calculate(operation));
}
// Function to read input from the user
// todo: add exit
fn read_input() -> String {
    println!("Enter your equation:");
    let mut input: String = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.to_string();
        }
        Err(_) => {
            println!("Error reading");
            return read_input();
        }
    }
}

// Convert infix notation into postfix
fn postfix(mut tokens: Vec<String>) -> Vec<String>{

   // generate a reverse polish notaion (postfix) using the shunting yard algorithm
    let mut output = Vec::new();
    let mut stack = Vec::new();

    // iterate through the tokens
    // if the token is a number, add it directly to the output
    while tokens.len() > 0 {
        if tokens[0].parse::<f64>().is_ok() {
            output.push(tokens[0].clone());
            tokens.remove(0);
        } else if

            // if the token is an operator:
            // exponentiation operator: push it directly to the stack since it is of highest priority
            tokens[0] == "^"
        {
            stack.push(tokens[0].clone());
            tokens.remove(0);
        } else if
            // multiplication or division:
            // check for if there are elemnts of higher or equal priority in the stack (^ * /)
            // pop them to the output then add the current token to the stack
            // if there aren't any, add the current token directly to the stack
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
            // it is a addition or subtraction:
            // check for if there are elemnts of higher priority in the stack (^ * / + -)
            // pop them to the output then add the current token to the stack
            // if there aren't any, add the current token directly to the stack
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
            // if the token is a left parenthesis push it to the stack
        } else if tokens[0] == "(" {
            stack.push(tokens[0].clone());
            tokens.remove(0);
            // if the token is a right parenthesis 
        } else if tokens[0] == ")" {
            while !stack.is_empty() && stack[stack.len() - 1] != "(" {
                output.push(stack.pop().unwrap());
            }
            if stack.is_empty() {
                println!("Error: Mismatched parentheses");
            } else {
            stack.pop().unwrap();
            tokens.remove(0);
            }
        }
    }
    while !stack.is_empty() {
      if stack[0] == "(" {
         println!("Error: Mismatched parentheses");
    } else {
        output.push(stack.pop().unwrap());
      }
    }
    return output;

}

// evaluates postfix notation
fn evalpostfix(postfix: Vec<String>) -> String {
    let mut stack: Vec<f64> = Vec::new();

    // for each token:
    for token in postfix {

        // if it's a number push it directly to the stack
        if token.parse::<f64>().is_ok(){
            stack.push(token.parse::<f64>().unwrap());
        } else {
        // if it's an operator, pop the last two elemants from the stack
        // where the first elemnt is the right operand and the second is the left operand
            let right = match stack.pop() {
                Some(val) => val,
                None => return String::from("Error: Invalid expression"),
            };
            let left = match stack.pop() {
                Some(val) => val,
                None => return String::from("Error: Invalid expression"),
            };
            if left.is_nan() || right.is_nan() {
                return String::from("Error: Invalid expression");
            }

            // perform the operation and push the result back to the stack
            let result = match token.as_str(){
                "+" => left + right,
                "-" => left - right,
                "/" => if right == 0.0 {
                    return String::from("Error: Cannont devide by zero");
                } else {
                    left / right
                },
                "*" => left * right,
                "^" => left.powf(right),
                _ => return String::from("Error: Invalid operator"),
            };
            stack.push(result);
        }
    }

    // the final result should be the only element left in the stack
    // if there is more than one element the given expression was invalid
    if stack.len() == 1 {
        return stack[0].to_string();
    } else {
        return String::from("Error: Invalid expression")
    }
}

fn calculate(input: String) -> String {
    // split the input using regex into tokens
    let re: Regex = Regex::new(r"(\d+|\+|\-|\*|\/|\^|\(|\))").unwrap();
    let tokens = re
        .find_iter(&input)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();

   // turn infix notaition into postfix notation
    let postfix = postfix(tokens);

   // evaluate the postfix notation
    evalpostfix(postfix)
}



