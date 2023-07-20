use ambient_api::{
    components::core::transform::translation,
    prelude::*,
};

use crate::components::{is_glider, glider_landvel, glider_hook_pos};

pub fn setup() {
    query((translation(), glider_landvel(), glider_hook_pos())).each_frame(|gliders|{
        for (glider,(gliderpos,landvel,hookpos)) in gliders {
            let to_hookpos = hookpos - gliderpos;
            let desired_landvel = to_hookpos.xy().normalize();
            let accellerp = 0.1;
            entity::mutate_component(glider, glider_landvel(), move |landvel|{
                *landvel = (1.-accellerp) * (*landvel) + (desired_landvel) * (accellerp);
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