use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        transform::{lookat_target, translation},
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
};

pub fn setup_camera1() -> EntityId {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn()
}