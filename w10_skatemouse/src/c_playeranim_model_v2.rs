use std::f32::consts::PI;

use ambient_api::{
    components::core::{
        app::name,
        transform::{local_to_parent, local_to_world, translation, rotation, scale},
        prefab::prefab_from_url,
        model::model_from_url,
        // primitives::cube,
        rendering::color,
    },
    prelude::*,
};

use crate::components::{is_glider, glider_landvel, glider_hook_pos, glider_forward};
use crate::components::glider_animodel; //clientside component

pub fn setup() {
    
    spawn_query(translation()).requires(is_glider()).bind(|gliders|{
        for (glider, pos) in gliders {
            let animodel = Entity::new()
                .with(name(), "Player Ani Model".to_string())
                .with(model_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
                .with(translation(), Vec3::ZERO)
                .with(rotation(), Quat::IDENTITY)
                .with(scale(), Vec3::splat(1.))
                .with(color(), random::<Vec3>().extend(1.0))
                .spawn();
            entity::add_component(glider, glider_animodel(), animodel);
        }
    });

    query((glider_animodel(), translation(), glider_forward())).each_frame(|gliders|{
        for (glider, (animodel, pos, fwd)) in gliders {
            entity::set_component(animodel, translation(), pos); // animodel position = my position
            entity::set_component(animodel, rotation(), Quat::from_rotation_arc_2d(vec2(0., 1.), fwd));
        }
    });

    change_query((is_glider(), translation())).track_change(translation()).bind(|gliders|{
        for (glider,(_,pos)) in gliders {
            // println!("Someone moved");
        }
    });
}