//
// Copyright 2021 Lim Chia Hau.
//
// Licensed under the GNU GENERAL PUBLIC LICENSE Version 3 <LICENSE or
// https://www.gnu.org/licenses/gpl-3.0.en.html>. This file may not be copied, 
// modified, or distributed except according to those terms.
//


extern crate system_health;

use system_health::system;

fn main() {
    match system_health::status() {
    	Ok(status) => system::notify(&status),
    	Err(e) => {
    		panic!("An unexpected error has occured. Details:\n{}",
    			e)
    	}
    }
}
