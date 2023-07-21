use ambient_api::ecs::GeneralQuery;
use ambient_api::entity::spawn;
use ambient_api::prelude::*;
use ambient_api::components::core::player::{user_id, local_user_id};
use crate::components::plr_glidercam;

pub fn build_query() -> GeneralQuery<(Component<EntityId>, Component<String>)> {
    query((plr_glidercam(), user_id())).build()
}

pub fn try_find_my_cam(query : GeneralQuery<(Component<EntityId>, Component<String>)>) -> Option<EntityId> {
    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();
    for (plr, (cam,uid)) in query.evaluate() {
        if uid == local_uid { return Some(cam); }
    }
    return None; // no camera found
}

pub fn my_cam_spawn_query(callback : fn(camera : EntityId)) {
    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();
    spawn_query((plr_glidercam(), user_id())).bind(move |cameras|{
        for (plr, (cam,uid)) in cameras {
            if uid == local_uid { callback(cam); }
        }
    });
}