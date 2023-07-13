use ambient_api::prelude::*;

#[main]
pub fn main() {
	messages::MouseDragging::subscribe(|source, msg| {
    	messages::MouseDragging{ ..msg }.send_local_broadcast(false);
	});
	messages::MouseClick::subscribe(|source, msg| {
    	messages::MouseClick{ ..msg }.send_local_broadcast(false);
	});
}
