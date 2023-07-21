use std::f32::consts::PI;

use ambient_api::prelude::*;
use ambient_api::components::core::transform::{translation, rotation};
use ambient_api::components::core::primitives::quad;

#[main]
pub fn main() {
    spawn_query(translation()).requires(quad()).bind(|ents|{
        for (ent,pos) in ents {
            println!("Clientside noticed quad @ {pos:?}");
            entity::mutate_component(ent, rotation(), |rot|*rot=Quat::from_rotation_z(PI * 0.5)**rot);
        }
    });
}
