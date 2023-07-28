use ambient_api::prelude::*;

mod x_camera;
mod s_matter_buoyancy;
mod s_matter_motors;
mod s_buoyancy_boats_example;
mod s_motor_boats_example;

#[main]
pub fn main() {
    s_matter_buoyancy::setup();
    s_matter_motors::setup();
    // s_buoyancy_boats_example::setup_example_1();
    s_motor_boats_example::setup_example_1();
}