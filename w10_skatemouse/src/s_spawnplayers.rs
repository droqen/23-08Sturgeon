use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        player::{player,user_id},
        primitives::{cube,quad},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, entity::spawn,
};

use crate::components::{is_glider,is_glidercam};
use crate::components::{plr_glider,plr_glidercam};
use crate::components::{glider_landvel};
use crate::components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};

pub fn setup() {
    spawn_query((player(), user_id())).bind(|players|{
        for (plr, (_,uid)) in players {

            // random land position, but on the ground
            let gliderpos = vec3(
                random::<f32>()*1.,
                random::<f32>()*5. + 10.,
                0.,
            );

            let glider = Entity::new()
                .with_merge(make_transformable())
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(user_id(), uid.clone())
                .with(cube(), ())
                .with(translation(), gliderpos)
                .spawn();
        
            entity::add_component(plr, plr_glider(), glider);

            let glidercam = Entity::new()
                .with_merge(make_transformable())
                .with_merge(make_perspective_infinite_reverse_camera())
                .with(aspect_ratio_from_window(), entity::resources())
                .with(is_glidercam(), ())
                .with(selfie_stick(), vec3(0., 10., 15.))
                .with(selfie_focus_ent(), glider.clone())
                .with(selfie_pitch(), 0.)
                .with(selfie_yaw(), 0.)
                .with(user_id(), uid.clone())
                .with(main_scene(), ())
                .spawn();

            entity::add_component(plr, plr_glidercam(), glidercam);
        }
    });
}