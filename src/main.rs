use std::io;

fn main() {
    loop {
        let first_value = 0;
        let second_value = 1;

        let input = get_input();
        if input == "stop" {
            break;
        }

        let input: u64 = input.trim().parse().expect("Error!");

        if input == 0 {
            println!("The input value must be greater than 0!");
        } else if input == 1 {
            println!("The n-th value of the Fibonacci sequence is: 0");
        } else if input == 2 {
            println!("The n-th value of the Fibonacci sequence is: 1");
        } else {
            match fibonacci_sequence(first_value, second_value, input) {
                Ok(result) => {
                    println!("The n-th value of the Fibonacci sequence is: {result}")
                }
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
    for _x in 0..n - 1 {
        let next_value = match first_value.checked_add(second_value) {
            Some(sum) => sum,
            None => return Err("Overflow occurred"),
        };
        first_value = second_value;
        second_value = next_value;
    }
    Ok(second_value)
}

fn get_input() -> String {
    let mut n = String::new();

    println!("Get the n-th number of the Fibonacci sequence.");
    println!("Which number do you want?");
    println!("Type 'stop' to exit the program.");

    io::stdin().read_line(&mut n).expect("Value is invalid!");
    n = n.trim().to_lowercase();
    n
}
