use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        player::{player,user_id},
        primitives::{cube,quad},
        physics::plane_collider,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, entity::spawn,
};

use components::{is_glider,is_glidercam};
use components::{plr_glider,plr_glidercam};
use components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};

mod s_selfie;
mod s_spawnplayers;
mod s_playerbehaviour;

#[main]
pub fn main() {

    crate::s_selfie::setup();
    crate::s_spawnplayers::setup();
    crate::s_playerbehaviour::setup();

    messages::MouseUVZero::subscribe(|src,msg|{
        let plr = src.client_entity_id().expect("Player has no entity");
        let glidercam = entity::get_component(plr, plr_glidercam()).expect("Player has no glidercam");
        entity::mutate_component(glidercam, selfie_yaw(), |yaw|*yaw=*yaw*0.5+0.5*msg.uvzero.x * 0.1);
        entity::mutate_component(glidercam, selfie_pitch(), |pitch|*pitch=*pitch*0.5+0.5*msg.uvzero.y * 0.1);
    });

    let tempquad = Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(plane_collider(), ())
        .spawn();

    messages::MouseDown::subscribe(move |_,msg|{
        if let Some(hit) = physics::raycast_first(msg.mouse_ray_origin, msg.mouse_ray_dir) {
            entity::set_component(tempquad, translation(), hit.position);
            // hit.distance, hit.entity
        }
    });
}
