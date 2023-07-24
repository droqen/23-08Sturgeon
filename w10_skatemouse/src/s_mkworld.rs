
fn lerp(a:f32,b:f32,t:f32) -> f32 { a*(1.-t)+b*t }

pub fn setup_world_entities() {
    // for i in 0..10+1 {
    //     let depth = i as f32 * 0.1;
    //     Entity::new()
    //         .with(name(), "Water layer".to_string())
    //         .with_merge(make_transformable())
    //         .with(quad(), ())
    //         .with(scale(), vec3(999., 999., 1.0))
    //         .with(transparency_group(), 1)
    //         .with(color(), vec4(
    //             lerp(0.1, 0.0, depth), 
    //             lerp(0.2, 0.0, depth), 
    //             lerp(0.7, 0.0, depth), 
    //             lerp(0.1, 1.0, depth),
    //         ))
    //         .with(translation(), vec3(0., 0., lerp(0.5, -0.5, depth)))
    //         .spawn();
    // }
    Entity::new()
        .with(name(), "Water (1)".to_string())
        .with_merge(make_transformable())
        .with(scale(), vec3(999., 999., 1.0))
        .with(transparency_group(), 1)
        .with(color(), vec4(0.4, 0.6, 0.9, 0.8))
        .with(quad(), ())
        .with(translation(), vec3(0., 0., 0.5))
        .spawn();

    // for i in 0..10 {
    //     Entity::new()
    //     .with(name(), "Underwater Darkness Layer".to_string())
    //     .with_merge(make_transformable())
    //     .with(scale(), vec3(999., 999., 1.0))
    //     .with(transparency_group(), 2)
    //     .with(color(), vec4(0., 0., 0., 0.1 * i as f32))
    //     .with(quad(), ())
    //     .with(translation(), vec3(0., 0., -0.5 - 0.5 * i as f32))
    //     .spawn();
    // }

        Entity::new()
            .with(name(), "Water (2)".to_string())
            .with_merge(make_transformable())
            .with(scale(), vec3(999., 999., 1.0))
            .with(transparency_group(), 1)
            .with(color(), vec4(0., 0., 0., 0.6))
            .with(quad(), ())
            .with(translation(), vec3(0., 0., -4.))
            .spawn();
        
        Entity::new()
            .with(name(), "Water (3)".to_string())
            .with_merge(make_transformable())
            .with(scale(), vec3(999., 999., 1.0))
            .with(transparency_group(), 1)
            .with(color(), vec4(0., 0., 0., 0.8))
            .with(quad(), ())
            .with(translation(), vec3(0., 0., -7.))
            .spawn();
        
            Entity::new()
                .with(name(), "Water (4)".to_string())
                .with_merge(make_transformable())
                .with(scale(), vec3(999., 999., 1.0))
                .with(transparency_group(), 1)
                .with(color(), vec4(0., 0., 0., 0.9))
                .with(quad(), ())
                .with(translation(), vec3(0., 0., -9.))
                .spawn();

    let pillars = Entity::new().with(name(), "Pillars".to_string()).spawn();
    // for i in 0..30 {
    //     let x = ((random::<u32>() % 10) as i32 - 5) * 2;
    //     let y = ((random::<u32>() % 10) as i32 - 5) * 2;
    //     entity::add_child(pillars, spawn_pillar_at(
    //         ivec2(x,y).as_vec2().extend(0.0),
    //         1.,
    //         3. + 5. * random::<f32>()
    //     ));
    // }
    
    for x in -10..10 {
        for y in -10..10 {
            if (
                (x % 2 == 0 || random::<f32>()<0.1)
                &&
                (y % 2 == 0 || random::<f32>()<0.1)
            ) && random::<f32>()<0.9 {
                let mut height = 1.0 + 0.8 * random::<f32>();
                let mut depth = 10.0;
                if random::<f32>() < 0.1 { height *= 1.0 + 5.0 * random::<f32>(); }
                entity::add_child(pillars, spawn_pillar_at(
                    ivec2(x,y).as_vec2().extend(0.0),
                    1.,
                    height,
                    depth,
                ));
            }
        }
    }
}

pub fn spawn_pillar_at(
    base_pos : Vec3,
    width : f32,
    height : f32,
    depth : f32,
) -> EntityId {
    return Entity::new()
        .with(name(), "Pillar".to_string())
        .with_merge(make_transformable())
        .with(cube(),())
        .with(cube_collider(), vec3(1.,1.,1.))
        .with(translation(), base_pos + vec3(0.,0.,height * 0.5 - depth * 0.5))
        .with(scale(), vec3(width,width,height + depth))
        .spawn();
}

use ambient_api::prelude::*;
use ambient_api::concepts::make_transformable;
use ambient_api::components::core::{
    app::name,
    transform::{translation,scale},
    physics::cube_collider,
    primitives::{cube, quad},
    rendering::{color,transparency_group},
};