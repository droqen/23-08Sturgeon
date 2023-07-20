use ambient_api::prelude::*;
use ambient_api::components::core::transform::{translation,scale};
use ambient_api::components::core::player::{user_id, local_user_id};
use ambient_api::concepts::make_transformable;
use crate::components::{
    is_hook_model,
    glider_hook_pos,
};

pub fn setup_hook_model_for_player_ent(plr : EntityId) {
    let hook_model = Entity::new()
        .with_merge(make_transformable())
        .with(is_hook_model(), ())
        .with(scale(), Vec3::splat(0.)) // scaled to zero.
        .spawn();
    change_query((glider_hook_pos(), user_id())).track_change(glider_hook_pos()).bind(|gliders|{
        let local_uid = entity::get_component(entity::resources(), local_user_id()).unwrap();
    });
}