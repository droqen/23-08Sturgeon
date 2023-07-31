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
mod c_hookmodel;
mod c_findmycam;
mod c_playeranim_model_v2_large;
mod c_debuglines;

const GLIDER_CAMERA_OFFSET : Vec3 = vec3(5., 5., 5.); // unused

#[main]
pub fn main() {
    // let local_uid =
    //     entity::get_component(entity::resources(), local_user_id()).unwrap();

    let findmycam_query = crate::c_findmycam::build_query();

    ambient_api::messages::Frame::subscribe(move |_|{
        let mouse_uv0: Vec2 = crate::c_pinutils::get_pin_mouse_uv0();
        let mouse_down: bool = crate::c_pinutils::get_pin_mouse_down();
        if mouse_down {
            if let Some(my_cam) = crate::c_findmycam::try_find_my_cam(findmycam_query) {
                let mouse_ray: Ray = crate::c_pinutils::get_pin_mouse_ray(my_cam);
                messages::MouseDown{ mouse_ray_origin: mouse_ray.origin, mouse_ray_dir: mouse_ray.dir }.send_server_unreliable();
            } else {
                println!("No my_cam found!");
            }
        }
        messages::MouseUVZero{ uvzero: mouse_uv0 }.send_server_unreliable();
    });

    crate::c_findmycam::my_cam_spawn_query(
        |cam|crate::c_debuglines::setup(cam));

    crate::c_hookmodel::setup_hook_model();
    crate::c_playeranim_model_v2_large::setup();

}
