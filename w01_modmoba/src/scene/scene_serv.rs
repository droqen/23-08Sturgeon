#[main]
pub fn main() {
	let _startingcamera = Entity::new()
		.with_merge(make_perspective_infinite_reverse_camera())
		.with(aspect_ratio_from_window(), EntityId::resources())
		.with_default(main_scene())
		.with(translation(), vec3(5.,5.,5.))
		.with(lookat_target(), vec3(0.,0.,0.))
		.spawn();
	let _startingquad = Entity::new()
		.with_merge(make_transformable())
		.with_default(quad())
		.with_default(plane_collider())
		.with(scale(), Vec3::splat(10.))
		.spawn();
}

use ambient_api::{
	components::core::{
		app::{main_scene},
		camera::{aspect_ratio_from_window},
		physics::{plane_collider,},
		primitives::{quad,},
		transform::{lookat_target, translation, scale,},
	},
	concepts::{make_perspective_infinite_reverse_camera,
		make_transformable,},
	prelude::*,
};