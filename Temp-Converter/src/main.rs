use std::io;

enum TempUnits
{
	Celsius,
	Fahrenheit,
	Kelvin
}

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

}

fn main()
{

	/*
		runtime:
			ask for queery in form X[T1] to [T2]
			where [T1] and [T2] are different TempUnits

			parse queery to determine which conversion to perform
				- if X[T1] to  [T1], dont do anything and display a short message saying as such
			
			display new temperature in form X[T1] is Y[T2]
	 */

	println!("press Ctrl+C to exit, or enter 'exit'");
	
	
	//main program loop	
	while true
	{
		print!("\nenter a queery (e.g. 25C to F)");
		let input: String = take_user_input();
		if input == "exit"
		{
			break;
		}

		println!("{}", input);

	}
	println!("exiting...");

}
