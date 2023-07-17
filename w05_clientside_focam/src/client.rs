use ambient_api::{
	components::core::{
		app::{main_scene, name,},
		camera::aspect_ratio_from_window,
        transform::{lookat_target, translation, rotation,},
	},
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, },
	prelude::*,
};

#[main]
pub fn main() {
    let ( camera, focal_point ) = yeah_like_make_a_camera_and_a_focal_point();
    catch_some_player_mouse_input_junk( camera.clone() );
	so_the_camera_just_rotates_round_the_focal_point( camera, focal_point );
    // apply that input to focam velocity
    // also convert that input to a world ray
    // send that ray to server
}

fn yeah_like_make_a_camera_and_a_focal_point() -> (EntityId, EntityId) {
	let camera_focal_point = Entity::new()
		.with_merge(make_transformable())
		.spawn();

    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
		.with(aspect_ratio_from_window(), EntityId::resources())
		.with(name(), "My Local Camera")
		.with_default(main_scene())
		.spawn();

	(camera, camera_focal_point)
}

fn catch_some_player_mouse_input_junk( camera : EntityId ) {
	let mut lmb_click_buffer = 0;
	let mut lmb_click_position = vec2(0., 0.);
	let mut prev_position = vec2(0., 0.);
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
                let ray = camera::screen_position_to_world_ray(camera, lmk_click_position);
        		messages::MouseClick{
                    screen_position: lmb_click_position,
                    ray_origin: ray.origin,
                    ray_dir: ray.dir,
                }.send_server_reliable();
        	}
        }
	});
}

fn so_the_camera_just_rotates_round_the_focal_point( camera : EntityId, focal_point : EntityId ) {

}