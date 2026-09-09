use std::io;

use crate::TempUnits::{Celsius, Fahrenheit, Kelvin};

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

	if first_exp == "" || second_exp == ""
	{
		println!("Input error, please try again");
		return;
	}

	calculate_conversion(first_exp, second_exp);
}

fn calculate_conversion(first: &str, second: &str)
{
	let first_unit: &str = &first.to_string()[first.len()-1..]; //get unit from first_exp
	let num_str: &str = &first.to_string()[..first.len()-1]; //split number from the unit
	let num = num_str.parse::<f64>().unwrap(); //parse number part as float

	let from: TempUnits;
	let convert_to: TempUnits;
	
	// at this point both parts of the equation are "X[T1]" and "[T2]", so getting the number should be trivial
	//match first unit so we know what to convert from
	match first_unit
	{
		"C" => from = Celsius,
		"F" => from = Fahrenheit,
		"K" => from = Kelvin,
		_ => //exit to main loop if none are found, would give a bad result
			{
				println!("Cannot match first part to a unit!");
				return;
			}, 
	}

	//match second unit so we know what to convert to
	match second
	{
		 "C" => convert_to = Celsius,
		 "F" => convert_to = Fahrenheit,
		 "K" => convert_to = Kelvin,
		 _ => //exit to main loop if none are found, would give a bad result
			{
				println!("Cannot match second part to a unit!");
				return;
			},
	}

	//match from and convert_to enums to print correct conversion, nested match statements to cover all possibilities
	match from
	{
		//match all conversions from Celsius
		Celsius => match convert_to
		{
			Celsius => println!("This does not need converting..."),
			Fahrenheit => println!("{} is {:.2}F", first, c_to_f(num)),
			Kelvin => println!("{} is {:.2}K", first, c_to_k(num)),
		}
		//match all conversions from fahrenheit
		Fahrenheit => match convert_to
		{
			Celsius => println!("{} is {:.2}C", first, f_to_c(num)),
			Fahrenheit => println!("This does not need converting..."),
			Kelvin => println!("{} is {:.2}K", first, f_to_k(num)),
		}
		//match all conversions from kelvin
		Kelvin => match convert_to
		{
			Celsius => println!("{} is {:.2}C", first, k_to_c(num)),
			Fahrenheit => println!("{} is {:.2}F", first, k_to_f(num)),
			Kelvin => println!("This does not need converting..."),
		}
	}
}

fn main()
{
	println!("press Ctrl+C to exit, or enter 'exit'");
	
	//main program loop	
	loop //this instead of while (true) because it does the exact same thing 
	{
		println!("\nsupported temps: C, F, K");
		println!("enter a conversion (e.g. 25C to F):");
		let input: String = take_user_input();

		if input.to_lowercase() == "exit"
		{
			break;
		}

		parse_input(input);
	}
	println!("exiting...");
}