use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        layout::space_between_items,
        rendering::color,
        transform::{lookat_target, translation},
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
};

#[main]
pub fn main() {
    let camera = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    TestElecom::el(camera).spawn_interactive();
}

#[element_component]
fn TestElecom(hooks: &mut Hooks, camera: EntityId) -> Element {
    let entities_with_hp = hooks.use_query((
        components::uiable_health(),
        components::uiable_health_max(),
        translation()
    ));

    UIBase.el().children(
        entities_with_hp.iter().map(|(_id,(hp, hpmax, pos))|{
            
            // Text::el(format!(
            //     "HP: {hp}/{hpmax} @ Pos: {pos}"
            // )).with(
            //     translation(),
            //     camera::world_to_screen(camera, *pos).extend(0.0),
            // )

            Centered::el([
                // Text::el(format!(
                //     "HP: {hp}/{hpmax} @ Pos: {pos}"
                // ))
                Text::el(format!(
                    "{hp}/{hpmax}"
                ))
            ]).with(
                translation(),
                camera::world_to_screen(camera, *pos + vec3(0.,0.,1.5)).extend(0.0),
            )
        }).collect::<Vec<_>>()
    )

    // position
    //     .map(|position| {
    //         Text::el(position.to_string())
    //             .with(
    //                 translation(),
    //                 camera::world_to_screen(camera, position).extend(0.0),
    //             )
    //             .with(color(), Vec4::ONE)
    //     })
    //     .unwrap_or_default()
}
