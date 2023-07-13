use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::quad,
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

use components::{focam_yaw_velocity};

#[main]
pub fn main() {
    messages::MouseDragging::subscribe(|_src, msg|{
        messages::SetYawVelocity{yaw_velocity: msg.drag_delta.x * 0.01}.send_local_broadcast(false);
    });

    query(focam_yaw_velocity()).each_frame(|focams|{
        for (focam, yawvel) in focams {
            entity::set_component(focam, focam_yaw_velocity(), yawvel * 0.97); // friction
        }
    });
}