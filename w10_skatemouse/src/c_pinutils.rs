use ambient_api::prelude::*;
use ambient_api::components::core::app::window_physical_size;

pub fn get_pin_mouse_uv0() -> Vec2 {
    let input = input::get();
    if let Some(window_size) = entity::get_component(entity::resources(), window_physical_size()) {
        let mouse_position_uv = vec2(
            input.mouse_position.x / window_size.x as f32,
            input.mouse_position.y / window_size.y as f32,
        );
        let mouse_position_center_offset = vec2(
            (mouse_position_uv.x*2.-1.).clamp(-1.,1.),
            (mouse_position_uv.y*2.-1.).clamp(-1.,1.),
        );
        mouse_position_center_offset
    } else { vec2(0.,0.) }
}

pub fn get_pin_mouse_ray() -> Ray {
    Ray{origin:vec3(0., 0., 0.), dir:vec3(1., 0., 0.)}
}