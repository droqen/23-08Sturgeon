pub fn setup_world_entities() {
    println!("Setup!");
    for i in 0..30 {
        println!("Setup {i:?}");
        let x = ((random::<u32>() % 10) as i32 - 5) * 2;
        let y = ((random::<u32>() % 10) as i32 - 5) * 2;
        spawn_pillar_at(
            ivec2(x,y).as_vec2().extend(0.0),
            1.,
            3. + 5. * random::<f32>()
        );
    }
}

pub fn spawn_pillar_at(
    base_pos : Vec3,
    width : f32,
    height : f32,
) -> EntityId {
    return Entity::new()
        .with(name(), "Pillar".to_string())
        .with_merge(make_transformable())
        .with(cube(),())
        .with(cube_collider(), vec3(1.,1.,1.))
        .with(translation(), base_pos + vec3(0.,0.,height * 0.5))
        .with(scale(), vec3(width,width,height))
        .spawn();
}

use ambient_api::prelude::*;
use ambient_api::concepts::make_transformable;
use ambient_api::components::core::{
    app::name,
    transform::{translation,rotation,scale},
    physics::cube_collider,
    primitives::cube,
};