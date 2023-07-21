use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        player::{player,user_id},
        primitives::quad,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    spawn_query((player(),user_id())).bind(|players|{
        for (plrent,(_,uid)) in players {
            let focam_driver = spawn_focam_driver_for_player(plrent, uid);
        }
    });
}


fn spawn_focam_driver_for_player(plrent : EntityId, uid : user_id) -> EntityId {
    let focam_cament = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        // .with(translation(), Vec3::splat(20.))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with(focuser(), ())
        .with(translation(), vec3(0., 0., 2.)) // +z = move camera up
        .with(focam_camera(), focam_cament)
        .with(focam_dist(), 30.)
        .with(focam_pitch(), 0.5) // positive: looking down (?)
        .with(focam_yaw(), 0.)
        .with(focam_yaw_velocity(), 0.01)
        .spawn()
}