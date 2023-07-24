use ambient_api::prelude::main;

mod s_selfie;
mod s_playergliderphysics_v2;
mod s_steering_and_camera;
mod s_mkworld;

#[main]
pub fn main() {

    crate::s_selfie::setup();
    crate::s_playergliderphysics_v2::setup();
    crate::s_steering_and_camera::setup_mouse_control();
    crate::s_mkworld::setup_world_entities();

}