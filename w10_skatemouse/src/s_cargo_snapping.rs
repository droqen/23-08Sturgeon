use ambient_api::prelude::*;
use ambient_api::components::core::{
    physics::physics_controlled,
    rendering::color,
    transform::{translation, scale},
};
use ambient_api::concepts::{make_transformable, make_sphere, };

use crate::components::{ is_glider, glider_cargo, };

pub fn setup() {
    spawn_query(is_glider()).bind(|gliders|{
        for (glider,_) in gliders {
            let cargo = Entity::new()
                .with_merge(make_transformable())
                .with_merge(make_sphere())
                .with(color(),random::<Vec3>().extend(1.0))
                .with(scale(), Vec3::splat(0.55))
                .spawn();
            entity::add_component(glider, glider_cargo(), cargo);
        }
    });

    query((translation(),glider_cargo())).requires(is_glider()).each_frame(|gliders|{
        for (glider,(glider_pos, cargo)) in gliders {
            // cargo anchor point
            if entity::has_component(cargo, physics_controlled()) {
                // physical cargo
            } else {
                // nonphysical cargo
                entity::mutate_component(cargo, translation(), |cargo_pos|{
                    *cargo_pos = *cargo_pos * 0.5 + 0.5 * (glider_pos + vec3(0., 0., 1.));
                });
            }
        }
    });
}