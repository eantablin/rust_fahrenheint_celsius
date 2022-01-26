use std::io;

fn main() {
    dialogue();
}

fn dialogue() {

    let mut u_input = String::new();

    println!("What's our return value? Fahrenheit or celsius?");
    
    io::stdin()
        .read_line(&mut u_input)
        .expect("Failed to read input");
    
    for i in u_input.chars() { // Yes, the hard way.
        if i == 'f'|| i == 'F' {
            println!("That's {} in celsius", f2c());
            break;
        } else if i == 'c' || i == 'C' {
            println!("That's {} in fahrenheit", c2f());
            break;
        }
    }

}

fn f2c() -> f64 {

    println!("Please input fahrenheit.");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let mut temp: f64 = match temp.trim().parse() {

        Ok(num) => num,
        Err(e) => {
            println!("Please input a number {}", e);
            f2c()
        }
    };
    
    temp = (temp - 32_f64) * (5_f64/9_f64); // Do some quick math, have to specify floats

    temp // Return manipulated variable
}

fn c2f() -> f64 {

    println!("Please input celsius.");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let mut temp: f64 = match temp.trim().parse() {

        Ok(num) => num,
        Err(e) => {
            println!("Please input a number {}", e);
            c2f()
        }
    };

    temp = (temp * 9_f64/5_f64) + 32_f64;

    temp
}