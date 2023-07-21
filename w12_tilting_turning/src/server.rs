use ambient_api::{
    components::core::{
        app::{main_scene, name},
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::line_vec;

#[main]
pub fn main() {
    // Entity::new()
    //     .with_merge(make_perspective_infinite_reverse_camera())
    //     .with(aspect_ratio_from_window(), EntityId::resources())
    //     .with_default(main_scene())
    //     .with(translation(), Vec3::ONE * 5.)
    //     .with(lookat_target(), vec3(0., 0., 0.))
    //     .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .spawn();

    Entity::new()
        .with(translation(), vec3(0., 0., 0.))
        .with(line_vec(), vec3(0., 0., 1.))
        .spawn();

    line(vec3(0., 0., 0.), vec3(3., 0., 0.), "X+");
    line(vec3(0., 0., 0.), vec3(0., 3., 0.), "Y+");
    line(vec3(0., 0., 0.), vec3(0., 0., 3.), "Z+");

    println!("Hello, Ambient!");
}

fn line(a:Vec3, b:Vec3, line_name:&str) -> EntityId {
    Entity::new()
        .with(translation(),(b+a)*0.5)
        .with(rotation(),Quat::from_rotation_arc(vec3(0., 1., 0.), (b-a).normalize()))
        .with(scale(),vec3(0.1,(b-a).length(), 0.1))
        .with(name(),line_name.to_string())
        .with(cube(), ())
        .spawn()
}
