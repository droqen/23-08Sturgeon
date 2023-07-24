use ambient_api::{
    components::core::{
        app::{main_scene,name,},
        camera::aspect_ratio_from_window,
        player::{player,user_id},
        primitives::cube,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use crate::components::{is_glider, is_glidercam};
use crate::components::{plr_glider, plr_glidercam};
use crate::components::{glider_landvel, glider_hook_pos};
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
                .with(name(), "Hook pos".to_string())
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(glider_hook_pos(), gliderpos)
                .with(user_id(), uid.clone())
                // .with(cube(), ()) // hidden. see c_playeranim.
                .with(translation(), gliderpos)
                .spawn();
        
            entity::add_component(plr, plr_glider(), glider);

            let glidercam = Entity::new()
                .with(name(), "Glider Camera".to_string() )
                .with_merge(make_perspective_infinite_reverse_camera())
                // .with_merge(make_transformable())
                .with(translation(), Vec3::splat(5.))
                .with(lookat_target(), Vec3::splat(0.))
                .with(main_scene(), ())
                .with(user_id(), uid.clone())
                .with(aspect_ratio_from_window(), entity::resources())
                .with(is_glidercam(), ())
                .with(selfie_stick(), vec3(7., 7., 10.))
                .with(selfie_focus_ent(), glider.clone())
                .with(selfie_pitch(), 0.)
                .with(selfie_yaw(), 0.)
                .spawn();

            entity::add_component(plr, plr_glidercam(), glidercam);
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