use ambient_api::prelude::*;
use ambient_api::input::Input;

pub fn spawn_focam() -> EntityId {
    set_up_the_camera();
    plus_some_ui_with_object_names();
}