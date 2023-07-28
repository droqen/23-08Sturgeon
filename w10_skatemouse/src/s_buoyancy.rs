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
    // is_matter,
    matter_gravity,
    // matter_is_buoyant,
    buoy_radius, buoy_water_level, buoy_max_force, buoy_max_friction, buoy_center_of_gravity_offset,

    dbg_sub,
};

pub fn setup() {
    query((
        translation(),
        rotation(),
        matter_gravity(),
        buoy_radius(),
        buoy_water_level(),
        buoy_max_force(),
        buoy_max_friction(),
        buoy_center_of_gravity_offset(),
        linear_velocity(),
        // angular_velocity(),
    )).each_frame(|buoys|{
        for(floaty_ent,
            (
            pos,
            rot,
            gravity,
            b_radius,
            b_water_level,
            b_max_force,
            b_max_friction,
            b_center,
            linvel,
            // angvel,
        )) in buoys {
            let submerged = get_submerged_percentage(pos.z, b_radius, b_water_level);

            entity::add_component(floaty_ent, dbg_sub(), submerged);

            if submerged > 0.0 {
                let b_force = vec3(0.,0.,1.) * b_max_force * submerged * delta_time();
                let b_friction = b_max_friction * submerged * delta_time();
                let b_friction_linvel_force = linvel * (-1.) * b_friction;
                let b_point : Vec3 = get_submerged_point(pos, b_radius, submerged);
                add_force_at_position(floaty_ent, pos + rot*b_center, b_force + b_friction_linvel_force, b_point);
                // let b_friction_angvel_force = angvel * (-1.) * b_friction;
                entity::mutate_component(floaty_ent, angular_velocity(), |angvel|*angvel *= (1.0 - b_friction));
            }
            entity::mutate_component(floaty_ent, linear_velocity(), |linvel|*linvel += vec3(0., 0., -gravity * delta_time()));
        }
    });
}

fn add_force_at_position(ent : EntityId, ent_center_of_gravity : Vec3, force : Vec3, force_point : Vec3) {
    entity::mutate_component(ent, linear_velocity(), |linvel|*linvel += force);
    entity::mutate_component(ent, angular_velocity(), |angvel|*angvel += (force_point - ent_center_of_gravity).cross(force));
}

// currently assumes object is basically a cube
// todo - assume object is sphere?
// todo - allow for any arbitrary shape?
// also todo - use a water plane?
fn get_submerged_percentage(z : f32, radius : f32, water_level : f32) -> f32 {
    if z < water_level-radius { return 1.00; }
    if z > water_level+radius { return 0.00; }
    if radius.abs() < 0.0001 { return 0.50; }
    let minusone_to_one = (water_level - z) / radius;
    let zero_to_one = minusone_to_one * 0.5 + 0.5;
    return zero_to_one;
}
fn get_submerged_point(pos : Vec3, radius : f32, submerged_percentage : f32) -> Vec3 {
    return pos + vec3(0., 0., radius * (2. * submerged_percentage - 1.));
}