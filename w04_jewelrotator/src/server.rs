use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::prefab_from_url,
        primitives::quad,
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, },
    prelude::*,
};

use components::{ focam_camera, focam_dist, focam_pitch, focam_yaw, focam_yaw_velocity, };

#[main]
pub fn main() {

    let focam_cament = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        // .with(translation(), Vec3::splat(20.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let focam_focus = Entity::new()
        .with_merge(make_transformable())
        .with(translation(), vec3(0., 0., 2.)) // +z = move camera up
        .with(focam_camera(), focam_cament)
        .with(focam_dist(), 30.)
        .with(focam_pitch(), 0.5) // positive: looking down (?)
        .with(focam_yaw(), 0.)
        .with(focam_yaw_velocity(), 0.01)
        .spawn();

    query((translation(), focam_camera(), focam_dist(), focam_pitch(), focam_yaw(), focam_yaw_velocity(),)).each_frame(|focams|{
        for (focam, (focus_pos, cam, dist, pitch, yaw, yawvel)) in focams {
            let cam_dir = Quat::from_rotation_z(yaw) * Quat::from_rotation_x(pitch) * vec3(0., dist, 0.);
            entity::set_component(focam, focam_yaw(), yaw + yawvel);
            entity::set_component(cam, translation(), focus_pos + cam_dir);
            entity::set_component(cam, lookat_target(), focus_pos);
        }
    });

    messages::SetYawVelocity::subscribe(move |_src, msg|{
        entity::set_component(focam_focus, focam_yaw_velocity(), msg.yaw_velocity);
    });

    Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/one_ring.glb").unwrap())
        .with(rotation(), Quat::from_rotation_y(0.5))
        // .with_default(quad())
        .spawn();

    println!("Hello, Ambient!");
}
