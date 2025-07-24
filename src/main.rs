use std::io;

fn main() {
    loop {
        let first_value = 0;
        let second_value = 1;
        let mut n = String::new();

        println!("Get the n-th number of the Fibonacci sequence.");
        println!("Which number do you want?");
        println!("Type 'stop' to exit the program.");

        io::stdin().read_line(&mut n).expect("Value is invalid!");

        n = n.trim().to_lowercase();

        if n == "stop" {
            break;
        }

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("There was an error!");
                break;
            }
        };

        if n == 0 {
            break;
        } else if n == 1 {
            println!("The n-th value of the Fibonacci sequence is: 0");
        } else {
            match fibonacci_sequence(first_value, second_value, n) {
                Ok(result) => println!("The n-th value of the Fibonacci sequence is: {result}"),
                Err(_) => println!("Overflow occurred! Number is too large to calculate."),
            }
        }
    }
}
fn fibonacci_sequence(
    mut first_value: u64,
    mut second_value: u64,
    n: u64,
) -> Result<u64, &'static str> {
    for _x in 0..n {
        let next_value = match first_value.checked_add(second_value) {
            Some(sum) => sum,
            None => return Err("Overflow occurred"),
        };
        first_value = second_value;
        second_value = next_value;
    }
    Ok(first_value)
}
