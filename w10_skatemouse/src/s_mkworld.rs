
fn lerp(a:f32,b:f32,t:f32) -> f32 { a*(1.-t)+b*t }

pub fn subscribe() {

    let parent_query: GeneralQuery<Component<()>> = query(is_mkworld_parent()).build();

    gen_water();

    crate::messages::GenerateWorld::subscribe(move |_src,_msg|{
        let parent = spawn_parent(parent_query);
        
        for cell in gen_cells_azoa() {
            entity::add_child(parent, spawn_building_stack_at(
                cell.as_vec2().extend(0.0) * 5.));
        }
    });
}


pub fn gen_cells_azoa() -> Vec<IVec2> {
    let mut cells = Vec::<IVec2>::new();
    for x in -10..10 {
        for y in -10..10 {
            if (
                (x % 2 == 0 || random::<f32>()<0.1)
                &&
                (y % 2 == 0 || random::<f32>()<0.1)
            ) && random::<f32>()<0.9 {
                cells.push(ivec2(x,y));
            }
        }
    }
    return cells;
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

pub fn spawn_building_stack_at(base_pos : Vec3) -> EntityId {

    let asset_index = 4; // random::<u8>()%5;
    let asset_string = vec![
        "MSH_Building_5x5x5_001", "MSH_Building_5x5x5_002",
        "MSH_Building_5x5x10_001", "MSH_Building_5x5x10_002",
        "building_mousehole"][asset_index as usize];
    let asset_height = match asset_index {
        0 | 1 => { 0. },
        2 | 3 => { 5. },
        4 | _ => { 0. },
    };

    dbg!(asset_string);

    let base_rot = Quat::from_rotation_z(PI*0.5*(random::<u8>()%4) as f32);

    let building_stack_base = Entity::new()
        .with(name(), "Building Stack".to_string())
        .with_merge(make_transformable())
        .with(model_from_url(), asset::url(format!("assets/{asset_string}.glb")).unwrap())
        // .with(cube(),())
        // .with(cube_collider(), vec3(5.,5.,10.))
        .with(translation(), base_pos + vec3(0., 0., asset_height))
        .with(rotation(), base_rot)
        // .with(scale(), vec3(width,width,height + depth))
        .spawn();

    for wall_local_rot_index in 0..3 {
        let wall_local_rot = Quat::from_rotation_z(PI*0.5*wall_local_rot_index as f32);
        entity::add_child(building_stack_base, Entity::new()
            .with(translation(), base_pos + base_rot * wall_local_rot * vec3(0., 1.75, 2.50))
            .with(rotation(), base_rot * wall_local_rot)
            .with(cube_collider(), vec3(5., 1.5, 5.))
            .spawn());
    }

    entity::add_child(building_stack_base, Entity::new()
        .with(quad(), ())
        .with(translation(), base_pos + vec3(0., 0., 2.25) + base_rot * vec3(2.25, 0., 0.))
        .with(rotation(), base_rot * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(5., 3., 1.))
        .with(color(), vec4(0., 0., 0., 0.5)) // black
        .with(transparency_group(), 6)
        .spawn());

    entity::add_child(building_stack_base, Entity::new()
        .with(quad(), ())
        .with(translation(), base_pos + vec3(0., 0., 2.25) + base_rot * vec3(2.15, 0., 0.))
        .with(rotation(), base_rot * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(5., 3., 1.))
        .with(color(), vec4(0., 0., 0., 0.5)) // black
        .with(transparency_group(), 6)
        .spawn());

    entity::add_child(building_stack_base, Entity::new()
        .with(quad(), ())
        .with(translation(), base_pos + vec3(0., 0., 2.25) + base_rot * vec3(2.05, 0., 0.))
        .with(rotation(), base_rot * Quat::from_rotation_y(PI * 0.5))
        .with(scale(), vec3(5., 3., 1.))
        .with(color(), Vec4::ZERO) // black
        .spawn());

    if random::<f32>() < 0.2 {
        spawn_building_ontop(building_stack_base, base_pos + vec3(0., 0., 5.));
    }

    return building_stack_base;
}

pub fn spawn_building_ontop(beneath_ent : EntityId, base_pos : Vec3) {
    let asset_index = random::<u8>()%4;
    let asset_string = vec![
        "MSH_Building_5x5x5_001", "MSH_Building_5x5x5_002",
        "MSH_Building_5x5x10_001", "MSH_Building_5x5x10_002",
    ][asset_index as usize];
    let asset_height = match asset_index {
        0 | 1 => { 0. },
        2 | 3 => { 5. },
        _ => { 0. },
    };
    
    let base_rot = Quat::from_rotation_z(PI*0.5*(random::<u8>()%4) as f32);

    // visual only: no collider needed
    let building_ontop = Entity::new()
        .with(name(), "Building On Top".to_string())
        .with(model_from_url(), asset::url(format!("assets/{asset_string}.glb")).unwrap())
        .with(translation(), base_pos + vec3(0., 0., asset_height))
        .with(rotation(), base_rot)
        .spawn();
    
    let building_separator = Entity::new()
        .with(name(), "Building Sep".to_string())
        .with(model_from_url(), asset::url("assets/MSH_Building_Seperator.glb").unwrap())
        .with(translation(), base_pos + vec3(0., 0., 0.))
        .with(rotation(), base_rot)
        .spawn();

    entity::add_child(beneath_ent, building_ontop);
    entity::add_child(beneath_ent, building_separator);

    if random::<f32>() < 0.1 {
        spawn_building_ontop(building_ontop, base_pos + vec3(0., 0., 5. + asset_height));
    }
}

pub fn spawn_parent(parent_query : GeneralQuery<Component<()>>)->EntityId {
    // despawn all old parents (& children)
    for(parent,_)in parent_query.evaluate(){entity::despawn_recursive(parent);}

    Entity::new()
        .with(name(), "Mkworld Parent".to_string())
        .with(is_mkworld_parent(), ())
        .spawn()
}

fn gen_water() {
    
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
}

use std::f32::consts::PI;

use ambient_api::entity::spawn;
use ambient_api::prelude::*;
use ambient_api::concepts::make_transformable;
use ambient_api::components::core::{
    app::name,
    transform::{translation,rotation,scale},
    prefab::prefab_from_url,
    model::model_from_url,
    physics::{cube_collider,visualize_collider},
    primitives::{cube, quad},
    rendering::{color,transparency_group},
};
use ambient_api::ecs::GeneralQuery;

use crate::components::is_mkworld_parent;