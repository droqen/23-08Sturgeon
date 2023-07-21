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

#[main]
pub fn main() {
    create_avi_when_player_connects();
    on_pin_move_set_avi_move_target();
    on_pin_imp_set_avi_to_imp_mode();
    when_imp_hits_a_breakable_object_destroy_it();
    create_some_breakable_objects_with_names();
}

fn create_some_breakable_objects_with_names() {
    for pos in vec![
        vec3(0., 0., 0.),
    ] {
        create_one_breakable_object(pos);
    }
}

fn when_imp_hits_a_breakable_object_destroy_it() {
    // todo!()
}

fn on_pin_imp_set_avi_to_imp_mode() {
    messages::PinPointImpulse::subscribe(|src,msg|{
        // todo!();
    });
}

fn on_pin_move_set_avi_move_target() {
    messages::PinPointMove::subscribe(|src,msg|{
        // todo!();
    });
}

fn create_avi_when_player_connects() {
    // todo!() // spawn query
}



fn create_one_breakable_object(pos : Vec3) {

}