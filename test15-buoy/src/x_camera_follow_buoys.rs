use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        transform::{lookat_target, translation},
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
};

use crate::components::buoy_submerged;

pub fn setup() {
    let camera_query = query((aspect_ratio_from_window(), lookat_target())).build();
    query(translation()).requires(buoy_submerged()).each_frame(move |buoys|{
        if let Some((camera,(_,_))) = camera_query.evaluate().first() {
            for (buoy, pos) in buoys {
                entity::mutate_component(*camera, translation(), |campos|*campos = 0.9**campos+0.1*(pos + Vec3::splat(5.)));
                entity::mutate_component(*camera, lookat_target(), |camlook|*camlook = 0.5**camlook+0.5*pos);
                //trying to look at all avail buoys
            }
        } else {
            println!("No camera found!");
        }
    });
}