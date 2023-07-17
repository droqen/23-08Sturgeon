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

use components::{ focam_camera, focam_dist, focam_pitch, focam_yaw, focam_yaw_velocity, focuser, focuser_focus_target, is_focusable, };

#[main]
pub fn main() {

    let (focam_cament, focam_focus) =
    spawn_focam_parts();

    so_like_focam_has_velocity_and_rotation();
    so_like_focuser_lerps_to_focus_target();
    // so_like_focus_alternates_between_two_rings_in_space();
    so_like_input_messages_change_focuser_focus_target();

    messages::SetYawVelocity::subscribe(move |_src, msg|{
        entity::set_component(focam_focus, focam_yaw_velocity(), msg.yaw_velocity);
    });

}

fn so_like_focam_has_velocity_and_rotation() {
    query((translation(), focam_camera(), focam_dist(), focam_pitch(), focam_yaw(), focam_yaw_velocity(),)).each_frame(|focams|{
        for (focam, (focus_pos, cam, dist, pitch, yaw, yawvel)) in focams {
            let cam_dir = Quat::from_rotation_z(yaw) * Quat::from_rotation_x(pitch) * vec3(0., dist, 0.);
            entity::set_component(focam, focam_yaw(), yaw + yawvel);
            entity::set_component(cam, translation(), focus_pos + cam_dir);
            entity::set_component(cam, lookat_target(), focus_pos);
        }
    });
}

fn so_like_focuser_lerps_to_focus_target() {
    query((focuser_focus_target(), translation())).each_frame(|focusers|{
        for (focuser, (focus_target_ent, current_pos)) in focusers {
            if let Some(focus_target_pos) = entity::get_component(focus_target_ent, translation()) {
                entity::set_component(focuser, translation(), current_pos*0.5 + focus_target_pos*0.5);
            } else {
                println!("focuser err! {focus_target_ent:?} does not have a translation() component");
            }
        }
    });
}

fn so_like_input_messages_change_focuser_focus_target() {
    let get_focuser = query(focuser()).build();
    messages::ClearFocusTarget::subscribe(move |_src,msg|{
        let focs = get_focuser.evaluate();
        if focs.len()>0 {
            let (foc,_) = focs[0];
            entity::remove_component(foc, focuser_focus_target());
        }
    });

    let get_focuser = query(focuser()).build();
    messages::SetFocusTarget::subscribe(move |_src,msg|{
        let focs = get_focuser.evaluate();
        if focs.len()>0 {
            let (foc,_) = focs[0];
            entity::add_component(foc, focuser_focus_target(), msg.focus_target);
        }
    });
}
    
fn spawn_focam_parts() -> (EntityId, EntityId) {
    let focam_cament = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        // .with(translation(), Vec3::splat(20.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let focam_focus = Entity::new()
        .with_merge(make_transformable())
        .with_default(focuser())
        .with(translation(), vec3(0., 0., 2.)) // +z = move camera up
        .with(focam_camera(), focam_cament)
        .with(focam_dist(), 30.)
        .with(focam_pitch(), 0.5) // positive: looking down (?)
        .with(focam_yaw(), 0.)
        .with(focam_yaw_velocity(), 0.01)
        .spawn();

    (focam_cament, focam_focus)
}

fn so_like_focus_alternates_between_two_rings_in_space() {
    let one_ring_model = Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/one_ring.glb").unwrap())
        .with(rotation(), Quat::from_rotation_y(0.5))
        // .with_default(quad())
        .with_default(is_focusable())
        .with(translation(), vec3(20., 0., 0.))
        .spawn();

    let other_ring_model = Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/one_ring.glb").unwrap())
        .with(rotation(), Quat::from_rotation_y(0.5))
        // .with_default(quad())
        .with_default(is_focusable())
        .with(translation(), vec3(200., 0., 0.))
        .spawn();
    
    let mut time = 0;

    ambient_api::messages::Frame::subscribe(move |_|{
        match time%100 {
            0 => messages::SetFocusTarget{focus_target: one_ring_model}.send_local_broadcast(true),
            50 => messages::SetFocusTarget{focus_target: other_ring_model}.send_local_broadcast(true),
            _ => {},
        };
        time += 1;
    });
}
