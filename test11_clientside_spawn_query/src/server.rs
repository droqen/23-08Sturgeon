use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::quad,
        player::{player, user_id},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*, entity::spawn,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), Vec3::ONE * 5.)
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    println!("Start!");

    mkquad(Vec2::ZERO);
    mkquad(vec2(random::<f32>()*-10., random::<f32>()*-10.));
    mkquad(vec2(random::<f32>()*-10., random::<f32>()*-10.));
    mkquad(vec2(random::<f32>()*-10., random::<f32>()*-10.));
    
    let mut frames = 0;

    ambient_api::messages::Frame::subscribe(move |_|{
        frames += 1;
        if frames < 50 {return;}
        frames = 0;
        mkquad(vec2(random::<f32>()*20.-10., random::<f32>()*20.-10.));
    });

    spawn_query((player(), user_id())).bind(|players|{
        for (player,(_,uid)) in players {
            println!("Player connected!");
            mkquad(vec2(random::<f32>()*20.-10., random::<f32>()*20.-10.));
        }
    });
}

fn mkquad(landpos:Vec2) {
    let pos = landpos.extend(0.0);
    Entity::new()
        .with_merge(make_transformable())
        .with(quad(), ())
        .with(translation(), pos)
        .spawn();
    println!("Serverside, made quad @ {pos:?}");
}
