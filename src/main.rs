use std::io;

// Celsius to Fahrenheight
fn c_to_f(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

// Fahrenheight to Celsius
fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0)/1.8
}

fn main() {
    loop {
        println!("Please give us a number, followed by either an 'f' or a 'c', like so: 100 f");
        let mut line_read = String::new();

        // This panics on failure
        io::stdin().read_line(&mut line_read)
            .expect("There was no stdio");
        // Create vector of words in given input
        let split = line_read.split_whitespace();
        let vec : Vec<&str> = split.collect();

        // Attempt to convert first argument to a float
        let num_val : f32 = match vec[0].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Was not able to parse the number!");
                continue
            }
        };

        // Check the second argument
        if vec[1].starts_with('c') {
            // Convert from celsius to Fahrenheight
            println!("In F, this is {}", c_to_f(num_val));
            break;
        }
        else {
            // Convert form Fahrenheight to Celsius
            println!("In C, this is {}", f_to_c(num_val));
            break;
        }
    }
}
