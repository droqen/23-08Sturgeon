#[main]
pub fn main() {
	messages::MouseLeftClick::subscribe(|_src, msg|{
		println!("Click!");
		if let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) {
			SpawnCross(hit.position);
		}
	});
}

fn SpawnCross(position : Vec3) {
	Entity::new()
		.with_merge(make_transformable())
		.with_default(cube())
		.with(translation(), position)
		.with(scale(), Vec3::splat(0.25))
		.spawn();
}

use ambient_api::{
	components::core::{
		primitives::{cube, },
		transform::{translation, scale, },
	},
	concepts::{make_transformable, },
	prelude::*,
};

// [messages.mouse_left_click.fields]
// ray_origin = { type = "Vec3" }
// ray_dir = { type = "Vec3" }
// player_id = { type = "EntityId" }