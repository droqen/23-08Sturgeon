#[main]
pub fn main() {
	ambient_api::messages::Frame::subscribe(|_|{
		println!("Unit talkin'");
	});
}

use ambient_api::prelude::*;