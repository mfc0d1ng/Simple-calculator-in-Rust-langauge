
use std::io;

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn parse_num(_user_input: &String) -> f64 {
    return _user_input.trim().parse().expect("Invalid number");
}

fn parse_op(_user_input: &String) -> char {
    return _user_input.trim().parse().expect("Invalid operator");
}

fn main()
{
    println!("Enter first number: ");
    let n1: f64 = parse_num(&user_input());
    
    println!("Enter second number:");
    let n2: f64 = parse_num(&user_input());

    println!("Enter operation (+ - * / %):");
    let op: char = parse_op(&user_input());
    
    let mut res = 0.0;
    let mut errflag: bool = false;

    if op == '+' {
        res = n1 + n2;
    }
    else if op == '-' {
        res = n1 - n2;
    }
    else if op == '*' {
        res = n1 * n2;
    }
    else if op == '/'{
        if n2 == 0.0 {
            println!("Error: division by zero!");
            errflag = true;
        }
        else {
            res = n1 / n2;
        }
    }
    else if op == '%'{
        if n2 == 0.0 {
            println!("Error: modulo by zero!");
            errflag = true;
        }
        else {
            res = n1 % n2;
        }
    }
    else {
        println!("Invalid operator '{op}'");
        errflag = true;
    }

    if !errflag {
        println!("\n {n1} {op} {n2} = {res}");
    }
    else {
        std::process::exit(1);
    }

}
