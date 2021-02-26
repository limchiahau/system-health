//
// Copyright 2021 Lim Chia Hau.
//
// Licensed under the GNU GENERAL PUBLIC LICENSE Version 3 <LICENSE or
// https://www.gnu.org/licenses/gpl-3.0.en.html>. This file may not be copied, 
// modified, or distributed except according to those terms.
//


use std::process::Command;
use std::string::FromUtf8Error;


pub fn status() -> Result<String, FromUtf8Error>{
	let output = Command::new("systemctl")
		.arg("status")
		.output()
		.expect("Unable to obtain system health");
		
	let system_status = String::from_utf8(output.stdout);
	
	system_status.map(|x| take_lines(2,&x))
}


fn take_lines(num_lines: usize, string: &str) -> String {
	let lines = string
		.lines()
		.take(num_lines);
	
	let mut result = String::new();
	
	for l in lines {
		result += &format!("{} |",l);
	}
	
	result
}


pub mod system {
	use std::process::Command;
	
	pub fn notify(msg: &str) {
		Command::new("notify-send")
			.arg(msg)
			.status()
			.expect("Notification failed");
	}
}


