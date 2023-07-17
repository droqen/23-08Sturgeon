#[main]
pub fn main() {
	let mut lmb_click_buffer = 0;
	let mut lmb_click_position = vec2(0., 0.);
	let mut prev_position = vec2(0., 0.);
	let main_camera_query = query(screenpin_camera()).build();
	ambient_api::messages::Frame::subscribe(move |_|{
		let (delta,input) = input::get_delta();
        if delta.mouse_buttons.contains(&MouseButton::Left) {
        	lmb_click_buffer = 30;
        	lmb_click_position = input.mouse_position;
        	prev_position = input.mouse_position;
        }
		if input.mouse_buttons.contains(&MouseButton::Left) {
			// if delta.mouse_position != input.mouse_position - prev_position {
			// 	println!("Mismatch! delta: {}, but mouse: {}", delta.mouse_position, input.mouse_position - prev_position);
			// }
			let delta_position = input.mouse_position - prev_position;
			if lmb_click_buffer <= 0 {
        		messages::MouseDragging{drag_delta: delta_position}.send_server_reliable();
				prev_position = input.mouse_position;
			} else if delta_position.length() > 5. {
				lmb_click_buffer = 0;
			}
		}
        if lmb_click_buffer > 0 { lmb_click_buffer -= 1; }
        if delta.mouse_buttons_released.contains(&MouseButton::Left) {
        	if lmb_click_buffer > 0 {
				let cameras = main_camera_query.evaluate();
				if cameras.len() > 0 {
					let (camera,_) = cameras[0];
					let ray = camera::screen_position_to_world_ray(camera, lmb_click_position);
					messages::MouseClick{
						screen_position: lmb_click_position,
						world_ray_origin: ray.origin,
						world_ray_dir: ray.dir,
					}.send_server_reliable();
				} else {
					println!("screenpin err - no screenpin_camera component found!");
				}
        	}
        }
	});
}

use ambient_api::{
	// components::core::{
    //     text::text,
	// },
	prelude::*,

    // concepts::{make_transformable},
};

use components::{screenpin_camera,};