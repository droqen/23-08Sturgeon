use ambient_api::prelude::main;

mod s_selfie;
mod s_spawnplayers;
mod s_playerbehaviour;
mod s_mousecontrol;
mod s_mkworld;

#[main]
pub fn main() {

    crate::s_selfie::setup();
    crate::s_spawnplayers::setup();
    crate::s_playerbehaviour::setup();
    crate::s_mousecontrol::setup_mouse_control();
    crate::s_mkworld::setup_world_entities();

}