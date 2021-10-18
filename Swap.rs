use std::io::{self, Write};
use std::fmt::Display;
use std::process;

fn main() 
{
    let mut num1: i32 = grab_input("Enter First Number ")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));
    let mut num2: i32 = grab_input("Enter Second Number ")
        .unwrap_or_else(|e| exit_err(&e, e.raw_os_error().unwrap_or(-1)))
        .trim()
        .parse()
        .unwrap_or_else(|e| exit_err(&e, 2));
    println!("Before Swapping, the value of first and second numbers are: {} and {}", num1, num2);
    num1 = num1 + num2;
    num2 = num1 - num2;
    num1 = num1 - num2;
    println!("After Swapping, the value of first and second numbers are: {} and {}", num1, num2);
}
fn grab_input(msg: &str) -> io::Result<String> 
{
    let mut buf = String::new();
    print!("{}: ", msg);
    try!(io::stdout().flush());
    try!(io::stdin().read_line(&mut buf));
    Ok(buf)
}
fn exit_err<T: Display>(msg: T, code: i32) -> ! 
{
    let _ = writeln!(&mut io::stderr(), "Error: {}", msg);
    process::exit(code)
}