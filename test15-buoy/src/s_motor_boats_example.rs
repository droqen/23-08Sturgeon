use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        model::model_from_url,
        physics::{physics_controlled,dynamic,cube_collider,visualize_collider,linear_velocity,angular_velocity,
            collider_from_url,},
        primitives::{cube, quad},
        rendering::{color, transparency_group, cast_shadows},
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use crate::components::{
    // is_matter,
    matter_gravity, matter_local_center,
    // matter_is_buoyant,
    buoy_radius, buoy_water_level, buoy_max_force, buoy_max_friction,
};

use crate::components::{
    motor_desired_dir, motor_throttle_power, motor_offset,
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

    make_boat( -2., 2. )
        .with(color(), vec4(1., 0., 0., 1.))
        .with(angular_velocity(), vec3(1000., 0., 0.))
        .with(linear_velocity(), vec3(0., 0., 10.))
        .spawn();
    make_boat( -1., 3. )
        .spawn();

    let boat = make_boat( 2., 0. )
        .spawn();
}

fn make_boat(screenwise_x : f32, z : f32) -> Entity {
    Entity::new()
        .with_merge(make_transformable())
        // .with_merge(make_buoyant_matter())
        .with(matter_gravity(), 9.81)
        .with(matter_local_center(), vec3(0.,0.,-0.5))
        .with(buoy_radius(), 0.5)
        .with(buoy_water_level(), 0.)
        .with(buoy_max_force(), 20.)
        .with(buoy_max_friction(), 3.)

        .with(motor_desired_dir(), vec3(-1., -1., 0.).normalize())
        .with(motor_throttle_power(), 0.1)
        .with(motor_offset(), vec3(0.0, 1.0, 0.0))

        .with(physics_controlled(), ())
        .with(dynamic(), true)
        .with(visualize_collider(), ())
        .with(linear_velocity(), Vec3::ZERO)
        .with(angular_velocity(), Vec3::ZERO)
        .with(translation(), vec3(screenwise_x, -screenwise_x, z))
        // .with(prefab_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
        .with(model_from_url(), asset::url("assets/MSH_Boat.glb").unwrap()) // creates model
        .with(collider_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
        // .with(cube_collider(), vec3(1., 2., 0.5))
}