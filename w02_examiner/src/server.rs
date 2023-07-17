use ambient_api::{
    components::core::{
        app::main_scene,
        camera::{active_camera, aspect_ratio_from_window,},
        prefab::prefab_from_url,
        physics::sphere_collider,
        primitives::quad,
        transform::{lookat_target, translation, rotation, },
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, ecs::GeneralQuery,
};

use components::{focam_yaw_velocity, is_focusable, };
use components::screenpin_camera;

#[main]
pub fn main() {

    first_my_cam_needs_a_screenpin_camera_component();

    so_like_theres_three_focusable_rings();
    and_like_clicking_on_a_ring_makes_it_the_focus_target();

    and_then_dragging_spins_focam();
    but_focam_doesnt_spin_forever();

}

fn first_my_cam_needs_a_screenpin_camera_component() {
    spawn_query(focam_yaw_velocity()).bind(|focams|{
        for (focam,_) in focams {
            entity::add_component(focam, screenpin_camera(), ());
        }
    });
}

fn so_like_theres_three_focusable_rings() {
    for ring_spawn_place in vec![
        vec3(0., 0., 0.),
        vec3(100., 0., -5.),
        vec3(51., 51., 20.)
    ] { so_just_make_a_focusable_ring_at_a_place(ring_spawn_place); }
}
fn and_like_clicking_on_a_ring_makes_it_the_focus_target() {
    messages::MouseClick::subscribe(move |_src, msg|{
        let click_ray = Ray{origin: msg.world_ray_origin, dir: msg.world_ray_dir};
        if let Some(raycast_hit) = physics::raycast_first(click_ray.origin, click_ray.dir) {
            println!("Yes, clicked on something");
        } else {
            println!("clicked on.. nothing!!!");
        }
    });
}

fn so_just_make_a_focusable_ring_at_a_place(place : Vec3) -> EntityId {
    return Entity::new()
        .with_default(is_focusable())
        .with(sphere_collider(), 1.)
        .with(prefab_from_url(), asset::url("assets/one_ring.glb").unwrap())
        .with(rotation(), Quat::from_rotation_y(0.5))
        .spawn();
}

fn and_then_dragging_spins_focam() {
    messages::MouseDragging::subscribe(|_src, msg|{
        messages::SetYawVelocity{yaw_velocity: msg.drag_delta.x * 0.01}.send_local_broadcast(false);
    });
}

fn but_focam_doesnt_spin_forever() {
    query(focam_yaw_velocity()).each_frame(|focams|{
        for (focam, yawvel) in focams {
            entity::set_component(focam, focam_yaw_velocity(), yawvel * 0.97); // friction
        }
    });
}
