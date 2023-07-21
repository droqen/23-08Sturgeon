use ambient_api::{
    components::core::{
        player::{player,user_id},
        primitives::{cube,quad},
        physics::plane_collider,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use crate::components::{is_glider,is_glidercam};
use crate::components::{plr_glider,plr_glidercam};
use crate::components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};
use crate::components::glider_hook_pos;

pub fn setup_mouse_control() {
    
    // mouse movement controls the camera

    crate::messages::MouseUVZero::subscribe(|src,msg|{
        let plr = src.client_entity_id().expect("MouseUVZero - Player has no entity");
        let glidercam = entity::get_component(plr, plr_glidercam()).expect("Player has no glidercam");
        entity::mutate_component(glidercam, selfie_yaw(), |yaw|*yaw=*yaw*0.5+0.5*msg.uvzero.x * 0.1);
        entity::mutate_component(glidercam, selfie_pitch(), |pitch|*pitch=*pitch*0.5+0.5*msg.uvzero.y * 0.1);
    });

    // creates the mouse control plane

    Entity::new()
        .with_merge(make_transformable())
        .with(plane_collider(), ())
        .spawn();

    // mouse clicking/holding does a raycast to the mouse control plane

    crate::messages::MouseDown::subscribe(move |src,msg|{
        let plr = src.client_entity_id().expect("MouseDown - Player has no entity");
        let glider = entity::get_component(plr, plr_glider()).expect("Player has no glidercam");
        for hit in physics::raycast(msg.mouse_ray_origin, msg.mouse_ray_dir) {
            if entity::has_component(hit.entity, plane_collider()) {
                entity::set_component(glider, glider_hook_pos(), hit.position);
                // entity::set_component(tempquad, translation(), hit.position);
                // hit.distance, hit.entity
            }
        }
    });
}
