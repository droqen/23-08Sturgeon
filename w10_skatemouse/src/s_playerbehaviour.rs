use ambient_api::{
    components::core::{
        transform::translation,
    },
    prelude::*,
};

use crate::components::{is_glider, glider_landvel};

pub fn setup() {
    query((is_glider(), translation(), glider_landvel())).each_frame(|gliders|{
        for (glider,(_,_,landvel)) in gliders {
            entity::mutate_component(glider, translation(), move |pos|{
                *pos += landvel.extend(0.0) * delta_time();
            });
        }
    });
}