use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/unit_cube.glb").unwrap())
        // .with_default(quad())
        .spawn();
    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with(translation(), vec3(1., -1., 0.))
    //     .with(prefab_from_url(), asset::url("assets/unit_cube.glb").unwrap())
    //     // .with_default(quad())
    //     .spawn();
    Entity::new()
        .with_merge(make_transformable())
        .with(translation(), vec3(2., -2., 0.))
        .with(prefab_from_url(), asset::url("assets/unit_cube.glb").unwrap())
        // .with_default(quad())
        .spawn();

    println!("Hello, Ambient!");
}
