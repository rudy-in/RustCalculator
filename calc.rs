// UwU
use std::io;
use minifb::{Key, Window, WindowOptions};

fn draw_expression(a: f64, b: f64, y: &str, result: f64) {
    let expression = format!("{} {} {} = {}", a, y, b, result);
    println!("{}", expression);
}

fn main() {
    let mut window = Window::new("Rust Calculator", 800, 600, WindowOptions::default())
        .expect("Unable to create window");

    loop {
        let mut input = String::new();

        println!("Enter number 1 (or 'exit' to end): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let a = input.trim();

        if a.to_lowercase() == "exit" {
            println!("Exiting the calculator.");
            break;
        }

        let mut input = String::new();

        println!("Enter number 2: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let b = input.trim();

        let mut input = String::new();

        println!("Enter operation (+, -, *, /): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let y = input.trim();

        let a: f64 = match a.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter valid numbers.");
                continue;
            }
        };

        let b: f64 = match b.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter valid numbers.");
                continue;
            }
        };

        let result = match y {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    println!("Error: Division by zero.");
                    continue;
                }
                a / b
            }
            _ => {
                println!("Invalid operation. Please enter +, -, *, or /.");
                continue;
            }
        };

        println!("Calculating the output...");

        // Simulate calculation time
        std::thread::sleep(std::time::Duration::from_secs(2));

        println!("Opening the live turtle window....");
        // Simulate opening turtle window
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Draw the expression in the graphics window
        draw_expression(a, b, y, result);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Handle graphics window events here
            window.update_with_buffer(&[0; 800 * 600 * 4]).expect("Failed to update window");
        }

        println!("Done");
    }
}
