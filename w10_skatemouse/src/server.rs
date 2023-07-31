use ambient_api::prelude::*;

mod s_selfie;
mod s_playergliderphysics_v2_large;
mod s_steering_and_camera;
mod s_mkworld;
mod s_cargo_snapping;
mod s_deliveries;
mod s_matter_buoyancy;

#[main]
pub fn main() {

    crate::s_selfie::setup();
    crate::s_playergliderphysics_v2_large::setup();
    crate::s_steering_and_camera::setup_mouse_control();
    crate::s_cargo_snapping::setup();
    crate::s_deliveries::setup();
    crate::s_matter_buoyancy::setup();

    crate::s_mkworld::subscribe();
    messages::GenerateWorld{gen:true}.send_local_broadcast(true);

}

