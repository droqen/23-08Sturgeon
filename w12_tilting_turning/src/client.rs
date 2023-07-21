use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        layout::space_between_items,
        rendering::color,
        transform::{lookat_target, translation},
        rect::{
            background_color, border_color, border_radius, border_thickness, line_from, line_to,
            line_width,
        },
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
};

mod c_nameui;

#[main]
pub fn main() {
    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    c_nameui::NameUI::el(camera).spawn_interactive();
}