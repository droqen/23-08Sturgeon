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
    spawn_query((player(), user_id())).bind(|players|{
        for (playerent,(_,_)) in players {
            entity::add_components(playerent,
                Entity::new()
                    .with(components::player_has_hover_target(), false)
                    .with_default(components::player_hover_target())
                    .with_default(components::player_hover_target_pos())
                    .with_default(components::player_hover_target_name())
            );
        }
    });

    // Entity::new()
    //     .with_merge(make_transformable())
    //     .with_default(quad())
    //     .with_default(plane_collider())
    //     .spawn();

    SpawnEnt(0., "Hello 0");
    SpawnEnt(5., "I'm at 20");
    SpawnEnt(-5., "Jimmy Beans");

    messages::Input::subscribe(|source, msg| {
        let Some(player_ent) = source.client_entity_id() else { println!("no player?"); return; };

        if let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) {
            // Get player entity id

            if let Some(ent_name) = entity::get_component(hit.entity, components::hoverable_name()) {
                // If hovered ent has a hover name
                entity::add_component(player_ent, components::player_has_hover_target(), true);
                entity::add_component(player_ent, components::player_hover_target(), hit.entity);
                entity::add_component(player_ent, components::player_hover_target_name(), ent_name);
                entity::add_component(player_ent, components::player_hover_target_pos(), hit.position);
            } else {
                // Else no hover
                entity::add_component(player_ent, components::player_has_hover_target(), false);
            }
        } else {
            // Else no hover
            entity::add_component(player_ent, components::player_has_hover_target(), false);
        }
    });
}

fn SpawnEnt(x:f32, name:&str) -> EntityId {
    Entity::new()
        .with(components::hoverable_name(), name.to_string())
        .with(cube_collider(), vec3(1., 1., 1.))
        .with(cube(), ())
        .with_merge(make_transformable())
        .with(scale(), vec3(1.5, 1.5, 1.5))
        .with(translation(), vec3(x, 0., 0.))
        .spawn()
}