#![feature(io)]

use std::fs::File;
use std::io::prelude::*;

fn main() {
	let file = File::open("input").expect("Could not open file");
	let digits = file.chars().filter_map(|c| c.expect("error decoding char").to_digit(10)).collect::<Vec<u32>>();
	let mut p1sum = 0;
	let mut p2sum = 0;
	let length = digits.len();
	let mut last:u32 = *digits.last().expect("Could not get last element!");
	for i in 0..length {
		if digits[i] == last {
			p1sum += digits[i];
		}
		last = digits[i];
		if digits[i] == digits[(i+length/2)%length] {
			p2sum += digits[i];
		}
	}
	println!("{:?}, {:?}", p1sum, p2sum);
}