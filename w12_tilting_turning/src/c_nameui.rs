use ambient_api::{
    components::core::{
        app::{main_scene, name},
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

#[element_component]
pub fn NameUI(hooks: &mut Hooks, camera: EntityId) -> Element {
    let named_ents = hooks.use_query((translation(), name()));
    Group::el(
        named_ents.iter().map(|(_id,(pos,name))|{
            Text::el(name)
                .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
        })
    )
}