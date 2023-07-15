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

use components::{focuser, focuser_focus_target, is_focusable, };

#[main]
pub fn main() {
    Entity::new()
        .with_default(focuser())
        .spawn();

    let get_focuser = query(focuser()).build();
    messages::ClearFocusTarget::subscribe(move |_src,msg|{
        let focs = get_focuser.evaluate();
        if focs.len()>0 {
            let (foc,_) = focs[0];
            entity::remove_component(foc, focuser_focus_target());
        }
    });

    let get_focuser = query(focuser()).build();
    messages::SetFocusTarget::subscribe(move |_src,msg|{
        let focs = get_focuser.evaluate();
        if focs.len()>0 {
            let (foc,_) = focs[0];
            entity::add_component(foc, focuser_focus_target(), msg.focus_target);
        }
    });

    debug();
}

fn debug() {
    let get_focuser = query(focuser()).build();
    let focs = get_focuser.evaluate();
    if focs.len()>0 {
        let (foc,_) = focs[0];
        messages::SetFocusTarget{focus_target:foc}.send_local_broadcast(true);
    }
} 