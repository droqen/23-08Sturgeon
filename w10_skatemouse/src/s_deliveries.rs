use ambient_api::prelude::*;
use ambient_api::components::core::{
    app::name,
    physics::physics_controlled,
    rendering::{color, transparency_group},
    transform::{translation, scale},
};
use ambient_api::concepts::{make_transformable, make_sphere, };

use crate::components::{ is_glider, glider_cargo, proximity_trigger, prox_is_pickup, prox_is_dropoff, };

pub fn setup() {
    make_prox(vec3(5., 2., 1.), 15.)
        .with(prox_is_pickup(), ())
        .spawn();
    make_prox(vec3(2., 5., 1.), 15.)
        .with(prox_is_dropoff(), ())
        .with(color(), vec3(1., 0., 1.).extend(0.25))
        .spawn();
    query((translation(), proximity_trigger()))
    .each_frame(|triggers|{
        for (prox_ent_id,(pos,radius)) in triggers {
            entity::set_component(prox_ent_id, proximity_trigger(), 0.5 + random::<f32>());
            for glider in entity::in_area(pos,radius).iter().filter(|ent_in_area|entity::has_component(**ent_in_area, is_glider())) {
                if entity::has_component(prox_ent_id, prox_is_pickup()) {
                    if !entity::has_component(*glider, glider_cargo()) {
                        crate::messages::AddCargoToGlider{glider: *glider}.send_local_broadcast(true);
                    }
                }
                if entity::has_component(prox_ent_id, prox_is_dropoff()) {
                    if entity::has_component(*glider, glider_cargo()) {
                        crate::messages::RemoveCargoFromGlider{glider: *glider}.send_local_broadcast(true);
                    }
                }
            }
        }
    });
}

fn make_prox(pos : Vec3, radius : f32) -> Entity {
    Entity::new()
        .with_merge(make_transformable())
        .with_merge(make_sphere())
        .with(name(), "Prox Sphere".to_string())
        .with(translation(), pos)
        .with(scale(), Vec3::splat(radius/4.))
        .with(color(), vec3(1., 1., 0.).extend(0.25))
        .with(transparency_group(), 3)
        .with(proximity_trigger(), radius)
        
}