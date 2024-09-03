

use std::io;

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}


fn soma (x: i32, y:i32) -> i32 {
	let sum = x + y ;
	println ! ("Sum: {}", sum );
	return 0;
}

fn subtr (x: i32, y:i32) -> i32 {
	let subtr = x - y ;
	println ! ("Sub: {}", subtr );
	return 0;
}

fn mult (x: i32, y:i32) -> i32 {
	let mult = x * y ;
	println ! ("Mult: {}", mult );
	return 0;
}

fn divi (x: i32, y:i32) -> i32 {
	let divi = x / y ;
	println ! ("Div: {}", divi );
	return 0;
}


fn main () {

//valores inicialmente chumbados

    println!("Enter the first number:");
    let a = read_number();

    println!("Enter the second number:");
    let b = read_number();

	println!("Enter the CHOICE number:");
    let choice = read_number();


match choice {
	1 => soma(a, b),
	2 => subtr(a,b),
	3 => mult(a,b),
	4 => divi(a,b),
	i32::MIN..=0_i32 | 3_i32..=i32::MAX => todo!(),

};





}