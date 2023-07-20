use ambient_api::prelude::*;
use ambient_api::components::core::transform::{translation, lookat_target};
use crate::components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};

pub fn setup() {
    query((selfie_stick(), selfie_focus_ent(), selfie_pitch(), selfie_yaw())).each_frame(|selfiesticks|{
        for (stick,(offset, target_ent, pitch, yaw)) in selfiesticks {
            let target_pos : Vec3 = entity::get_component(
                target_ent,
                 translation()
            ).expect("Selfie stick focus ent does not have a 'translation' component");
            entity::add_component(stick, translation(), target_pos + Quat::from_rotation_y(-yaw) * Quat::from_rotation_x(pitch) * offset);
            entity::add_component(stick, lookat_target(), target_pos);
        }
    });
}