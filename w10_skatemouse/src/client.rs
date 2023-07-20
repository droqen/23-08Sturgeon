use ambient_api::{
    components::core::{
        app::{main_scene, window_physical_size},
        camera::aspect_ratio_from_window,
        player::{local_user_id, player, user_id},
        transform::{translation,lookat_target},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::is_glider;
use components::is_local_player_camera;

mod c_pinutils;

const GLIDER_CAMERA_OFFSET : Vec3 = vec3(5., 5., 5.);

#[main]
pub fn main() {
    let local_uid =
        entity::get_component(entity::resources(), local_user_id()).unwrap();

    ambient_api::messages::Frame::subscribe(|_|{
        let mouse_uv0: Vec2 = crate::c_pinutils::get_pin_mouse_uv0();
        let mouse_ray: Ray = crate::c_pinutils::get_pin_mouse_ray();
        messages::MouseUVZero{ uvzero: mouse_uv0 }.send_server_unreliable();
    });

    // spawn_query((is_glider(),translation(),user_id())).bind(move |gliders|{
    //     for (glider,(_,pos,uid)) in gliders {
    //         println!("{}, {}", &uid, &local_uid);
    //         if &uid == &local_uid {
    //             // create a local camera for this glider!
    //             let cament : EntityId = Entity::new()
    //                 .with_merge(make_perspective_infinite_reverse_camera())
    //                 .with(aspect_ratio_from_window(), EntityId::resources())
    //                 .with(main_scene(), ())
    //                 .with(translation(), pos + GLIDER_CAMERA_OFFSET)
    //                 .with(lookat_target(), pos)
    //                 .with(is_local_player_camera(), ())
    //                 .spawn();
    //         }
    //     }
    // });

}
