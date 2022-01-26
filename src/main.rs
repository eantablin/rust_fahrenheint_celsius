use std::io;

fn main() {
    println!("Let's convert to celsius");
    println!("Enter temp in fahrenheit: ");

    let u_input = f2c();
    println!("That's {} in celsius", u_input);
}

fn f2c() -> f64 {
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