use ambient_api::{
    components::core::{
        app::{main_scene,name,},
        camera::aspect_ratio_from_window,
        model::model_from_url,
        player::{player,user_id},
        // primitives::cube,
        physics::{
            // cube_collider,
            sphere_collider,
            physics_controlled, dynamic, linear_velocity, angular_velocity, visualize_collider,
            // collider_from_url,
        },
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
    // glam::EulerRot,
};

use crate::components::local_forward;
use crate::components::{is_glider, is_glidercam};
use crate::components::{plr_glider, plr_glidercam};
use crate::components::{
    glider_landvel, glider_steer_vector, glider_hook_pos,
    glider_stat_max_speed, glider_stat_handling, glider_stat_reverse_speed,
    glider_forward, glider_forward_rotvel,};
use crate::components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};




use crate::components::{matter_gravity, matter_local_center,
    buoy_max_force, buoy_max_friction, buoy_radius, buoy_water_level, buoy_submerged,
};

pub fn setup() {
    spawn_query((player(), user_id())).bind(|players|{
        for (plr, (_,uid)) in players {

            // random land position, but on the ground
            let gliderpos = vec3(
                random::<f32>()*1.,
                random::<f32>()*5. + 20.,

                random::<f32>()*1. + 1., // drop from a slight height
            );

            let glider = Entity::new()
                .with_merge(make_transformable())
                .with(physics_controlled(), ())
                .with(dynamic(), true)
                // .with(cube_collider(), vec3(1.0, 1.0, 0.25))
                .with(sphere_collider(), 0.70)
                // .with(model_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
                // .with(collider_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
                // .with(visualize_collider(), ())

                .with(linear_velocity(), vec3(0., 0., 3.)) // toss up
                .with(angular_velocity(), Vec3::ZERO)

                .with(name(), "Hook pos".to_string())
                .with(local_forward(), vec3(0., 1., 0.))
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(glider_steer_vector(), vec2(0., -1.))
                .with(glider_hook_pos(), gliderpos.truncate().extend(0.))
                .with(glider_forward(), vec2(0., 1.))
                .with(glider_forward_rotvel(), 10.)

                .with(glider_stat_max_speed(), 20.0)
                .with(glider_stat_handling(), 2.0)
                .with(glider_stat_reverse_speed(), 5.0)

                .with(user_id(), uid.clone())
                // .with(cube(), ()) // hidden. see c_playeranim.
                .with(translation(), gliderpos)

                .with(matter_gravity(), 9.81)
                .with(matter_local_center(), vec3(0.,0.,-1.))
                .with(buoy_radius(), 1.)
                .with(buoy_water_level(), 0.)
                .with(buoy_max_force(), 20.)
                .with(buoy_max_friction(), 3.)

                .spawn();
            
            entity::add_component(plr, plr_glider(), glider);

            let glidercam = Entity::new()
                .with(name(), "Glider Camera".to_string() )
                .with_merge(make_perspective_infinite_reverse_camera())
                // .with_merge(make_transformable())
                .with(translation(), Vec3::splat(5.))
                .with(lookat_target(), Vec3::splat(0.))
                .with(main_scene(), ())
                .with(user_id(), uid.clone())
                .with(aspect_ratio_from_window(), entity::resources())
                .with(is_glidercam(), ())
                .with(selfie_stick(), vec3(7., 7., 10.))
                .with(selfie_focus_ent(), glider.clone())
                .with(selfie_pitch(), 0.)
                .with(selfie_yaw(), 0.)
                .spawn();

            entity::add_component(plr, plr_glidercam(), glidercam);
        }
    });
    
    query((
        is_glider(),
        translation(),
        rotation(),
        glider_forward(),
        glider_steer_vector(),
        glider_landvel(),
        linear_velocity(),
    )).each_frame(|gliders|{
        for (glider, (
            _,
            pos,
            rot,
            fwd,
            steer_vec2,
            landvel,
            vel,
        )) in gliders {
            let angle_to_fwd = notnan_or_zero(fwd.angle_between(steer_vec2));
            // let angle_to_fwd = notnan_or_zero(fwd.truncate().angle_between(steer_vec2)) * (steer_vec2.length() * 1.5 - 0.5).clamp(0.0, 1.0);

            let accellin = 0.5 * delta_time();
            let accellerp = 0.02;
            let friction = 0.01;

            let stat_max_speed =     entity::get_component(glider, glider_stat_max_speed())
                .unwrap_or(10.0);
            let stat_handling =     entity::get_component(glider, glider_stat_handling())
                .unwrap_or(1.0);
            let stat_reverse_speed =    entity::get_component(glider, glider_stat_reverse_speed())
                .unwrap_or(4.0);

            let mut target_speed = stat_max_speed * steer_vec2.length();

            if target_speed > 0. {
                let backup_angle = 0. + stat_handling;
                let full_speed_angle = 0. + stat_handling/2.;
                let backup_amount = invlerp(full_speed_angle, backup_angle, angle_to_fwd.abs()).clamp(0., 1.);

                target_speed *= lerp(stat_max_speed, stat_reverse_speed, backup_amount) / stat_max_speed;

                if let Some(submerged) = entity::get_component(glider, buoy_submerged()) {
                    if submerged < 0.50 {
                        target_speed *= 0.25 + 0.75 * submerged * 2.;
                    }
                } else {
                    target_speed = 0. // no movement is allowed? maybe allow twitches or something
                }
            }

            let live_target_landvel : Vec2 = steer_vec2 * target_speed;
            
            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                *linvel *= 1.-friction;
                let to_live_target_landvel : Vec2 = live_target_landvel - linvel.truncate();
                if to_live_target_landvel.length_squared() < accellin * accellin {
                    *linvel = (
                        live_target_landvel
                    ).extend(linvel.z);
                } else {
                    *linvel = (
                        linvel.xy()
                        + to_live_target_landvel.clamp_length_max(accellin)
                        + to_live_target_landvel * accellerp
                    ).extend(linvel.z);
                }

                entity::set_component(glider, glider_landvel(), linvel.xy());
            });

            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                linvel.z -= 9.81 * delta_time(); // gravity
                if pos.z - 1.0 < 0. {
                    linvel.z *= 0.95; // water has drag
                    linvel.z += -(pos.z - 1.0) * delta_time() * (15. + 10. * random::<f32>()); // buoyancy slightly unpredictable
                }
            });

            // entity::mutate_component(glider, angular_velocity(), |angvel|{
            //     let live_target_zangvel = angle_to_fwd * 3.0;
            //     let to_live_target_zangvel = (live_target_zangvel - angvel.z).clamp(-0.5, 0.5);
            //     angvel.z += to_live_target_zangvel * target_speed * delta_time(); // turn!
            // });
        }
    });

    query(glider_forward_rotvel()).each_frame(|gliders|{
        for(glider,fwd_rotvel) in gliders {
            entity::mutate_component(glider, glider_forward(), |forward|*forward=forward.rotate(Vec2::from_angle(fwd_rotvel*delta_time())));
        }
    });
}

fn invlerp(from : f32, to : f32, value : f32) -> f32 { (value - from) / (to - from) }
fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }
fn remap(origFrom : f32, origTo : f32, targetFrom : f32, targetTo : f32, value : f32) {
    let rel = invlerp(origFrom, origTo, value);
    lerp(targetFrom, targetTo, rel);
}
fn remap_clamped(origFrom : f32, origTo : f32, targetFrom : f32, targetTo : f32, value : f32) {
    let rel = invlerp(origFrom, origTo, value).clamp(0., 1.);
    lerp(targetFrom, targetTo, rel);
}
fn notnan_or_zero(notnan : f32) -> f32 {
    if notnan.is_nan() { 0. } else { notnan }
}