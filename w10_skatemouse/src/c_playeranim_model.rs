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

use crate::components::{is_glider, glider_landvel, glider_hook_pos};
use crate::components::{glider_animodel, animodel_lookdir, animodel_yaw, animodel_yawvel}; //clientside components

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
                .with(animodel_lookdir(), vec3(1., 0., 0.))
                .with(animodel_yaw(), 0.)
                .with(animodel_yawvel(), 10.) // spawn spinning
                .spawn();
            entity::add_component(glider, glider_animodel(), animodel);
        }
    });

    query((glider_animodel(), translation(), glider_landvel(), glider_hook_pos())).each_frame(|gliders|{
        for (glider, (animodel, pos, landvel, hookpos)) in gliders {
            entity::set_component(animodel, translation(), pos); // animodel position = my position
            // let model_yaw = entity::get_component(animodel, animodel_yaw()).expect("Animodel has no 'yaw'");
            let a : Vec3 = landvel.extend(0.0);
            let b : Vec3 = hookpos - pos;
            let mut desired_lookdir : Vec3 = Vec3::ZERO;
            if a.length_squared() > 0.2 { desired_lookdir += a.normalize(); }
            if b.length_squared() > 0.2 { desired_lookdir += b.normalize(); }
            if desired_lookdir.length_squared() > 0.2 {
                desired_lookdir = desired_lookdir.normalize();
                entity::set_component(animodel, animodel_lookdir(), desired_lookdir);
                entity::set_component(animodel, rotation(), Quat::from_rotation_arc_2d(vec2(0., 1.), desired_lookdir.truncate()));
                // let desired_yawvel = shortest_yaw_delta(model_yaw, dir2yaw(desired_lookdir));
                // println!("Yaw vel from yaw{:?} to yaw{:?} is vel{:?}", model_yaw, dir2yaw(desired_lookdir), desired_yawvel);
                // entity::mutate_component(animodel, animodel_yawvel(), move |yawvel|{
                //     *yawvel *= 0.99;
                //     *yawvel += 0.01 * desired_yawvel;
                // });
            } else {
                // apply friction, basically.
                // entity::mutate_component(animodel, animodel_yawvel(), |yawvel|*yawvel *= 0.8); // high friction
            }
            // entity::set_component(animodel, animodel_lookdir(), landvel.extend(0.0));
        }
    });

    // query((rotation(), animodel_yaw(), animodel_yawvel())).each_frame(|models|{
    //     for (model,(rot,yaw,yawvel)) in models {
    //         entity::set_component(model, rotation(), yaw2rot(yaw));
    //         // entity::mutate_component(model, animodel_yaw(), |yaw|*yaw += yawvel);
    //         // entity::set_component(model, rotation(),  
    //         // yaw2rot(yaw) *
    //         //     // trying for a tilt left/right
    //         //     Quat::from_rotation_x(yawvel*0.01)
    //         // );
    //     }
    // });

    change_query((is_glider(), translation())).track_change(translation()).bind(|gliders|{
        for (glider,(_,pos)) in gliders {
            // println!("Someone moved");
        }
    });
}

fn yaw2rot(yaw : f32)->Quat { Quat::from_rotation_z(yaw) }
// fn yaw2dir(yaw : f32)->Vec3 { yaw2rot(yaw) * vec3(0., -1., 0.) }
// fn dir2yaw(dir : Vec3)->f32 { dir.x.atan2(-dir.y) }
// fn shortest_yaw_delta(a:f32, b:f32)->f32 {
//     let c = (b-a).rem_euclid(PI*2.);
//     if c > PI {PI*2.-c} else {c}
// }