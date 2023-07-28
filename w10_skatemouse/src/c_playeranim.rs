use std::f32::consts::PI;

use ambient_api::{
    components::core::{
        app::name,
        transform::{local_to_parent, local_to_world, translation, rotation, scale},
        primitives::cube,
        rendering::color,
    },
    prelude::*,
};

use crate::components::{is_glider, glider_landvel, glider_hook_pos, glider_forward};
use crate::components::{glider_animodel, animodel_lookdir}; //clientside components

pub fn setup() {

    spawn_query(translation()).requires(is_glider()).bind(|gliders|{
        for (glider, pos) in gliders {
            let animodel = Entity::new()
                .with(name(), "Player Ani Model".to_string())
                .with(cube(), ())
                .with(translation(), Vec3::ZERO)
                .with(rotation(), Quat::IDENTITY)
                .with(scale(), vec3(0.60, 0.60, 1.0))
                .with(color(), random::<Vec3>().extend(1.0))
                .with(animodel_lookdir(), vec3(1., 0., 0.))
                // .with(local_to_parent(), Mat4::IDENTITY)
                .spawn();

            // entity::add_child(glider, animodel);
            // entity::add_component(glider, local_to_world(), Mat4::IDENTITY);
            entity::add_component(glider, glider_animodel(), animodel);
        }
    });


    query((glider_animodel(), translation(), glider_landvel(), glider_hook_pos(), glider_forward())).each_frame(|gliders|{
        for (glider, (animodel, pos, landvel, hookpos, fwd)) in gliders {
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

fn yaw2rot(yaw : f32)->Quat { Quat::from_rotation_z(yaw) }