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
