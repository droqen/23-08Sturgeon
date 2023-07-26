use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        model::model_from_url,
        physics::{physics_controlled,dynamic,cube_collider,visualize_collider,linear_velocity,angular_velocity,},
        primitives::{cube, quad},
        rendering::{color, transparency_group, cast_shadows},
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use crate::components::{
    // is_matter,
    matter_gravity,
    // matter_is_buoyant,
    buoy_radius, buoy_water_level, buoy_max_force, buoy_max_friction, buoy_center_of_gravity_offset,
};

pub fn setup_example_1() {

    // spawn water plane
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(),())
        .with(transparency_group(),1)
        .with(color(), vec4(0.3, 0.5, 0.9, 0.5))
        .with(scale(), vec3(999., 999., 1.))
        .spawn();

    make_boat( 0., 2. )
        .with(color(), vec4(1., 0., 0., 1.))
        .with(angular_velocity(), vec3(1000., 0., 0.))
        .with(linear_velocity(), vec3(0., 0., 10.))
        .spawn();
    make_boat( 4., 0. )
        .spawn();
}

fn make_boat(screenwise_x : f32, height : f32) -> Entity {
    Entity::new()
        .with_merge(make_transformable())
        // .with_merge(make_buoyant_matter())
        .with(matter_gravity(), 9.81)
        .with(buoy_center_of_gravity_offset(), vec3(1.,0.,-0.5))
        .with(buoy_radius(), 1.)
        .with(buoy_water_level(), 0.)
        .with(buoy_max_force(), 20.)
        .with(buoy_max_friction(), 3.)

        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(visualize_collider(), ())
        .with(linear_velocity(), Vec3::ZERO)
        .with(angular_velocity(), Vec3::ZERO)
        .with(translation(), vec3(screenwise_x, -screenwise_x, height))
        .with(model_from_url(), asset::url("assets/MSH_Boat.glb").unwrap()) // creates model
        .with(cube_collider(), vec3(1., 2., 1.))
}