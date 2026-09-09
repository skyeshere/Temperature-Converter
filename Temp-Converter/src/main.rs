use std::io;

use crate::TempUnits::{Celsius, Fahrenheit, Kelvin};

#[derive(Debug)]
enum TempUnits
{
	Celsius,
	Fahrenheit,
	Kelvin
}


//conversion functions
fn c_to_f(c: f64) -> f64
{
	return c * (9. /5.) + 32.0;
}

fn c_to_k(c: f64) -> f64
{
	return c + 273.15;
}

fn f_to_c(f: f64) -> f64
{
	return (f - 32.0) * (5./9.)
}

fn f_to_k(f: f64) -> f64
{
	//convert to c, then add 273.15
	let c = f_to_c(f);
	return c_to_k(c);
}

fn k_to_c(k: f64) -> f64
{
	return k - 273.15;
}

fn k_to_f(k: f64) -> f64
{
	//convert to c, then convert to f
	let c = k_to_c(k);
	return c_to_f(c);
}

fn take_user_input() -> String
{
	let mut buffer = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut buffer).expect("failed to read line");

	return buffer.trim().to_string();
}

fn parse_input(input: String)
{
	let mut first_exp: &str = "";
	let mut second_exp: &str = "";
	//traverse input string
	for i in 0..input.len()
	{
		let c: char = input.as_bytes()[i] as char;

		if c == 't' && input.as_bytes()[i+1] as char == 'o'
		{
			first_exp = &input[0..i].trim();
			second_exp = &input[i+2..].trim();
			break;
		}
		
	}

	calculate_conversion(first_exp, second_exp);
}

fn calculate_conversion(first: &str, second: &str)
{
	
	let first_unit: &str = &first.to_string()[first.len()-1..]; //get unit from first_exp
	let first_number_str: &str = &first.to_string()[..first.len()-1];
	let first_number = first_number_str.parse::<f64>().unwrap(); //parse number part as float
	let mut from: TempUnits = Celsius;
	let mut convert_to: TempUnits = Celsius;
	
	// at this point both parts of the equation are "X[T1]" and "[T2]"", so getting the number should be trivial

	//match first unit so we know what to convert from
	match first_unit
	{
		"C" => from = Celsius,
		"F" => from = Fahrenheit,
		"K" => from = Kelvin,
		_ => println!("Something went wrong!")
	}

	//match second unit so we know what to convert to
	match second
	{
		 "C" => convert_to = Celsius,
		 "F" => convert_to = Fahrenheit,
		 "K" => convert_to = Kelvin,
		 _ => println!("Something went wrong!")
	}

	//match from and convert_to enums to print correct conversion
	match from
	{
		Celsius => match convert_to
		{
			Celsius => println!("This does not need converting..."),
			Fahrenheit => println!("{:.2}F", c_to_f(first_number)),
			Kelvin => println!("{:.2}K", c_to_k(first_number)),
		}

		Fahrenheit => match convert_to
		{
			Celsius => println!("{:.2}C", f_to_c(first_number)),
			Fahrenheit => println!("This does not need converting..."),
			Kelvin => println!("{:.2}K", f_to_k(first_number)),
		}

		Kelvin => match convert_to
		{
			Celsius => println!("{:.2}C", k_to_c(first_number)),
			Fahrenheit => println!("{:.2}F", k_to_f(first_number)),
			Kelvin => println!("This does not need converting..."),
		}
	}
}

fn main()
{
	println!("press Ctrl+C to exit, or enter 'exit'");
	
	
	//main program loop	
	while true
	{
		println!("\nsupported temps: C, F, K");
		println!("enter a conversion (e.g. 25C to F):");
		let input: String = take_user_input();

		if input == "exit"
		{
			break;
		}

		parse_input(input);
	}
	println!("exiting...");

}
	/*
		runtime:
			ask for queery in form X[T1] to [T2]
			where [T1] and [T2] are different TempUnits

			parse queery to determine which conversion to perform
				- if X[T1] to  [T1], dont do anything and display a short message saying as such
			
			display new temperature in form X[T1] is Y[T2]
	 */
