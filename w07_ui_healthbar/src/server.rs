use ambient_api::{
    components::core::{
        physics::{cube_collider, plane_collider},
        player::{player, user_id},
        primitives::{cube, quad},
        transform::{translation,scale},
    },
    concepts::make_transformable,
    prelude::*,
};

#[main]
pub fn main() {
    // spawn_query((player(), user_id())).bind(|players|{
    //     for (playerent,(_,_)) in players {
    //         entity::add_components(playerent,
    //             Entity::new()
    //                 .with(components::player_has_hover_target(), false)
    //                 .with_default(components::player_hover_target())
    //                 .with_default(components::player_hover_target_pos())
    //                 .with_default(components::player_hover_target_name())
    //         );
    //     }
    // });

    SpawnEnt(0., 50,100);
    SpawnEnt(5., 100,100);
    SpawnEnt(-5., 0,30);

    query(ui_guy_move_target()).each_frame(|movers|{
        for (mover_ent,(move_target)) in movers{
            entity::mutate_component(mover_ent, translation(), move |pos| {
                let to_target = move_target - *pos;
                if to_target.length() < 0.1 {
                    *pos += to_target;
                    entity::set_component(mover_ent, ui_guy_move_target(), vec3(
                        random::<f32>() * 10.-5.,
                        random::<f32>() * 10.-5.,
                        random::<f32>() * 10.-5.,
                    ));
                } else {
                    *pos += to_target.clamp_length_max(0.1);
                }
            });
        }
    });

}

fn SpawnEnt(x:f32, hp:i32, hp_max:i32) -> EntityId {
    Entity::new()
        .with(components::uiable_health(), hp)
        .with(components::uiable_health_max(), hp_max)
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(cube(), ())
        .with_merge(make_transformable())
        .with(scale(), vec3(1.5, 1.5, 1.5))
        .with(translation(), vec3(x, 0., 0.))
        .with(ui_guy_move_target(), vec3(x, 5., 0.))
        .spawn()
}

use components::ui_guy_move_target;