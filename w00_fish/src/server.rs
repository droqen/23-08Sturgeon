

mod sceneloader;

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        // .with(translation(), Vec3::ONE * 5.)
        .with(translation(), Vec3::ONE * 1.) // zoom camera in
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    let groundplane = Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with_default(plane_collider())
        .with(scale(), Vec3::splat(5.))
        .spawn();

    let nodes = sceneloader::test_get_nodes();
    sceneloader::debug_nodes(&nodes);
    for (nodename, node) in nodes {
        match &nodename.as_str(){
            &"plane"=>{
                entity::set_component(groundplane, translation(), node.pos.unwrap());
            },
            _=>{
                if nodename.as_str().contains("snickers_") {
                    if let Some(node_path) = node.path {
                    Entity::new()
                        .with(name(), "~SNICKERS".to_string())
                        .with_merge(make_transformable())
                        // .with_default(physics_controlled())
                        // .with_default(linear_velocity())
                        // .with_default(angular_velocity())
                        // .with(dynamic(), true)
                        // .with(collider_from_url(), asset::url("assets/".to_owned()+&node_path).unwrap())
                        .with(model_from_url(), asset::url("assets/".to_owned()+&node_path).unwrap())
                        .with(translation(), node.pos.unwrap())
                        .with(rotation(), node.rot.unwrap())
                        .with(scale(), node.siz.unwrap())
                        .spawn();
                    } else {
                        println!("Node path doesn't work for object {:?}?", nodename);
                    }
                }
            },
        }
    }

    // let cube = Entity::new()
    //     .with(name(), "~CUBE".to_string())
    //     .with_merge(make_transformable())
    //     .with_default(cube())
    //     // .with_default(visualize_collider())
    //     .with_default(physics_controlled())
    //     // .with_default(cast_shadows())
    //     .with_default(linear_velocity())
    //     .with_default(angular_velocity())
    //     .with(cube_collider(), Vec3::ONE)
    //     .with(dynamic(), true)
    //     .with(translation(), vec3(0., 0., 5.))
    //     .with(rotation(), Quat::IDENTITY)
    //     .with(scale(), vec3(0.5, 0.5, 0.5))
    //     .with(color(), Vec4::ONE)
    //     .spawn();



    println!("Hello, Ambient!");
}
use ambient_api::{
    components::core::{
        app::{main_scene, name, },
        camera::aspect_ratio_from_window,
        model::model_from_url,
        physics::{cube_collider, plane_collider, collider_from_url, dynamic, physics_controlled,
            linear_velocity, angular_velocity, visualize_collider, },
        // prefab::{prefab_from_url },
        primitives::{cube,quad},
        rendering::{color},
        transform::{lookat_target, translation, rotation, scale},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};