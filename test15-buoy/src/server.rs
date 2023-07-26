use ambient_api::prelude::*;

mod x_camera;
mod s_buoyancy;
mod s_buoyancy_boats_example;

#[main]
pub fn main() {
    s_buoyancy::setup();
    s_buoyancy_boats_example::setup_example_1();
}