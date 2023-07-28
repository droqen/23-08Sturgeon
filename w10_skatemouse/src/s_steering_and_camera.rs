use ambient_api::{
    components::core::{
        primitives::quad,
        physics::plane_collider,
        transform::translation,
    },
    concepts::make_transformable,
    prelude::*,
};

use crate::components::{plr_glider,plr_glidercam};
use crate::components::{selfie_pitch, selfie_yaw};
use crate::components::{is_glider, glider_desired_landvel, glider_hook_pos};

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
        .with(translation(), vec3(0., 0., -2.))
        .spawn();

    // mouse clicking/holding does a raycast to the mouse control plane
    // and sets the appropriate player's 'glider_hook_pos'

    crate::messages::MouseDown::subscribe(move |src,msg|{
        let plr = src.client_entity_id().expect("MouseDown - Player has no entity");
        let glider = entity::get_component(plr, plr_glider()).expect("Player has no glidercam");
        for hit in physics::raycast(msg.mouse_ray_origin, msg.mouse_ray_dir) {
            if entity::has_component(hit.entity, plane_collider()) {
                entity::set_component(glider, glider_hook_pos(), hit.position.truncate().extend(0.0));
            }
        }
    });

    // set the player's desired_landvel
    
    query((translation(), glider_hook_pos())).each_frame(|gliders|{
        for (glider,(gliderpos,hookpos)) in gliders {
            let to_hookpos : Vec3 = hookpos - gliderpos;
            if to_hookpos.xy().length_squared() < 0.01 {
                entity::set_component(glider, glider_desired_landvel(),
                    Vec2::ZERO
                );
            } else {
                entity::set_component(glider, glider_desired_landvel(),
                    to_hookpos.xy().clamp_length(0.1, 5.0) * 2.0
                );
            }
        }
    });
}
