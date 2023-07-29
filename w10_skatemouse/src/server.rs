use ambient_api::prelude::main;

mod s_selfie;
mod s_playergliderphysics_v2_b;
mod s_steering_and_camera;
mod s_mkworld;
mod s_cargo_snapping;
mod s_deliveries;
mod s_matter_buoyancy;

#[main]
pub fn main() {

    crate::s_selfie::setup();
    crate::s_playergliderphysics_v2_b::setup();
    crate::s_steering_and_camera::setup_mouse_control();
    crate::s_mkworld::setup_world_entities();
    crate::s_cargo_snapping::setup();
    crate::s_deliveries::setup();
    crate::s_matter_buoyancy::setup();

}

