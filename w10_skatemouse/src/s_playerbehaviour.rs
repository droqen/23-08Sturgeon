use ambient_api::{
    components::core::transform::translation,
    prelude::*,
};

use crate::components::{is_glider, glider_landvel, glider_hook_pos};

pub fn setup() {
    query((translation(), glider_landvel(), glider_hook_pos())).each_frame(|gliders|{
        for (glider,(gliderpos,landvel,hookpos)) in gliders {
            let to_hookpos : Vec3 = hookpos - gliderpos;
            let desired_landvel : Vec2;
            if to_hookpos.length_squared() < 0.001 {
                desired_landvel = Vec2::ZERO;
            } else {
                // let desired_landvel = to_hookpos.xy().normalize();
                desired_landvel = to_hookpos.xy().clamp_length(0.1, 5.0) * 1.7;
            }
            let accellin = 1.0 * delta_time();
            let accellerp = 0.05;
            let friction = 0.01;
            entity::mutate_component(glider, glider_landvel(), move |landvel|{
                *landvel *= (1.-friction);
                let to_desired_landvel : Vec2 = desired_landvel - *landvel;
                *landvel += to_desired_landvel.clamp_length_max(accellin) + to_desired_landvel * accellerp;
            });
        }
    });

    query((is_glider(), translation(), glider_landvel())).each_frame(|gliders|{
        for (glider,(_,_,landvel)) in gliders {
            entity::mutate_component(glider, translation(), move |pos|{
                *pos += landvel.extend(0.0) * delta_time();
            });
        }
    });
}