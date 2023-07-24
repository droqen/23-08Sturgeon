use ambient_api::{
    components::core::{
        app::{main_scene, name},
        camera::aspect_ratio_from_window,
        layout::space_between_items,
        physics::sphere_collider,
        rendering::color,
        transform::{lookat_target, translation},
        rect::{
            background_color, border_color, border_radius, border_thickness, line_from, line_to,
            line_width,
        },
        layout::{width,height,},
    },
    concepts::make_perspective_infinite_reverse_camera,
    prelude::*,
};

use crate::components::glider_hook_pos;

pub fn setup(camera : EntityId) {
    // HookLineUI::el(camera).spawn_interactive();
    // PhysCircleUI::el(camera).spawn_interactive();
}

// DISPLAYS DEBUG CIRCLE ON SPHERE COLLIDERS
#[element_component]
pub fn PhysCircleUI(hooks: &mut Hooks, camera: EntityId) -> Element {
    let sphere_ents = hooks.use_query((translation(), sphere_collider()));
    Group::el(
        sphere_ents.iter().map(|(_id,(pos,_radius))|{
            Centered::el([
            Rectangle::el()
                        .with(line_width(), 2.)
                        .with(width(), 20.)
                        .with(height(), 20.)
            ])
            .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
            // Line::el()
            //     .with(line_from(), camera::world_to_screen(camera, *pos).extend(0.))
            //     .with(line_to(), camera::world_to_screen(camera, *hook_pos).extend(0.))
            //     .with(background_color(), vec4(1.,0.,1.,1.))
            //     .with(line_width(), 2.)
            // // Text::el(name)
            // //     .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
        })
    )
}

// DISPLAYS DEBUG LINE OF PLAYER'S PATH...
#[element_component]
pub fn HookLineUI(hooks: &mut Hooks, camera: EntityId) -> Element {
    let hooky_ents = hooks.use_query((translation(), glider_hook_pos()));
    Group::el(
        hooky_ents.iter().map(|(_id,(pos,hook_pos))|{
            Line::el()
                .with(line_from(), camera::world_to_screen(camera, *pos).extend(0.))
                .with(line_to(), camera::world_to_screen(camera, *hook_pos).extend(0.))
                .with(background_color(), vec4(1.,0.,1.,1.))
                .with(line_width(), 2.)
            // Text::el(name)
            //     .with(translation(), camera::world_to_screen(camera, *pos).extend(0.))
        })
    )
}

// DISPLAYS NAMES OF ALL ENTITIES THAT HAVE NAMES...
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