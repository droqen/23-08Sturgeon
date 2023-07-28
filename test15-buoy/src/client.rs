use ambient_api::prelude::*;
use ambient_api::components::core::transform::translation;
use ambient_api::components::core::physics::angular_velocity;
use components::buoy_submerged;

mod x_camera;
mod x_camera_follow_buoys;

#[main]
pub fn main() {
    let camera = x_camera::setup_camera1();
    // x_camera_follow_buoys::setup();
    ShowDbgInfo::el(camera).spawn_interactive();
}

#[element_component]
fn ShowDbgInfo(hooks: &mut Hooks, camera: EntityId) -> Element {
    let entities_with_hp = hooks.use_query((
        translation(),
        buoy_submerged(),
        angular_velocity(),
    ));

    UIBase.el().children(
        entities_with_hp.iter().map(|(_id,(pos, sub, angvel))|{
            Centered::el([
                Text::el(format!(
                    "{}", (*sub * 100.) as i32
                    // "{}", angvel
                ))
            ]).with(
                translation(),
                camera::world_to_screen(camera, *pos + vec3(0.,0.,1.5)).extend(0.0),
            )
        }).collect::<Vec<_>>()
    )
}
