use ambient_api::prelude::*;
use ambient_api::input::Input;

#[main]
pub fn main() {
    set_up_the_camera();
    plus_some_ui_with_object_names();
}

fn handle_mouse_drag(delta_position: Vec2) -> () {
    // TODO: rotate camera
    // messages::MouseDragging{drag_delta: delta_position}.send_server_reliable();
}
fn handle_mouse_full_click(mouse_position: Vec2, mouse_ray: Ray) -> () {
    
}

fn so_ya_move_with_the_mouse(input : &Input) {
    dbg!(input);
    // todo!()
}
fn need_some_camera_controls_too() {
    // todo!()
}
fn clicking_the_button_launches_an_impulse() {
    // todo!()
}

fn set_up_the_camera() {
    // todo!()
}
fn plus_some_ui_with_object_names() {
    // todo!()
}