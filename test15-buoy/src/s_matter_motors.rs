use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        physics::{physics_controlled,dynamic,cube_collider,visualize_collider,linear_velocity,angular_velocity,},
        primitives::quad,
        rendering::{color, transparency_group},
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use crate::components::{
    matter_local_center,
    motor_desired_dir, motor_throttle_power, motor_offset,
};

pub fn setup() {
    query((
        translation(),
        rotation(),
        matter_local_center(),
        motor_throttle_power(),
        motor_offset(),
        motor_desired_dir(),
    )).each_frame(|cars|{
        for (car,(
            pos,
            rot,
            mass_center,
            throttle_power,
            motor_offset,
            desired_dir,
        )) in cars {
            let motor_pos = rot * motor_offset;
            add_force_at_position(car, rot * mass_center, desired_dir * throttle_power * delta_time(), rot * motor_pos)
        }
    });
}

fn add_force_at_position(ent : EntityId, ent_mass_center_pos : Vec3, force : Vec3, force_point : Vec3) {
    entity::mutate_component(ent, linear_velocity(), |linvel|*linvel += force);
    entity::mutate_component(ent, angular_velocity(), |angvel|*angvel += (force_point - ent_mass_center_pos).cross(force));
}