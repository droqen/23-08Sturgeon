use ambient_api::prelude::*;
use ambient_api::components::core::{
    transform::{translation,rotation,scale},
    player::{user_id, local_user_id},
    primitives::cube,
};
use ambient_api::concepts::make_transformable;
use crate::components::{
    is_hook_model, hook_model_pos, hook_model_recency,
    glider_hook_pos,
};

pub fn setup_hook_model() {
    let hook_model = Entity::new()
        .with_merge(make_transformable())
        .with(cube(),())
        .with(is_hook_model(), ())
        .with(hook_model_pos(), Vec3::ZERO)
        .with(hook_model_recency(), 0.)
        .with(scale(), Vec3::ZERO) // scaled to zero.
        .spawn();

    query((hook_model_pos(),hook_model_recency())).each_frame(|hookmodels|{
        for (hookmodel, (target_pos, recency)) in hookmodels {
            entity::mutate_component(hookmodel, hook_model_recency(), move |recency|{
                if *recency > 0. {
                    *recency -= delta_time();
                    if *recency > 0. {
                        entity::set_component(hookmodel, translation(), target_pos + vec3(0., 0., *recency * 0.5));
                        entity::set_component(hookmodel, scale(), vec3(0.1, 0.1, *recency));
                        entity::mutate_component(hookmodel, rotation(), |rot|*rot = Quat::from_rotation_z(0.1 * delta_time()) * *rot);
                    } else {
                        *recency = 0.;
                        entity::set_component(hookmodel, scale(), Vec3::ZERO);
                    }
                }
            });
        }
    });

    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();
    
    change_query((glider_hook_pos(), user_id())).track_change(glider_hook_pos()).bind(move |gliders|{
        for (glider,(hook_pos, uid)) in gliders {
            if uid == local_uid {
                entity::set_component(hook_model, hook_model_pos(), hook_pos);
                entity::set_component(hook_model, hook_model_recency(), 1.);
            }
        }
    });
}