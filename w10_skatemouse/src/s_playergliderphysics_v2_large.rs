use std::f32::consts::PI;

use ambient_api::{
    components::core::{
        app::{main_scene,name,},
        camera::aspect_ratio_from_window,
        player::{player,user_id},
        primitives::cube,
        physics::{cube_collider, sphere_collider, physics_controlled, dynamic, linear_velocity, angular_velocity, visualize_collider, },
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
    glam::EulerRot,
};

use crate::components::{is_glider, is_glidercam};
use crate::components::{plr_glider, plr_glidercam};
use crate::components::{glider_landvel, glider_steer_vector, glider_hook_pos,
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
                random::<f32>()*1. + 5.,
                random::<f32>()*5. + 25.,

                random::<f32>()*1. + 1., // drop from a slight height
            );

            let glider = Entity::new()
                .with_merge(make_transformable())
                .with(physics_controlled(), ())
                .with(dynamic(), true)
                .with(sphere_collider(), 6.66 * 0.25) // fills 66% of the corridor
                .with(visualize_collider(), ())
                .with(linear_velocity(), vec3(0., 0., 0.)) // start with no velocity
                .with(angular_velocity(), Vec3::ZERO)

                .with(name(), "Glider".to_string())
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(glider_steer_vector(), vec2(0., -1.))
                .with(glider_hook_pos(), gliderpos.truncate().extend(0.))
                .with(glider_forward(), vec2(0., 1.))
                .with(glider_forward_rotvel(), 0.)

                .with(user_id(), uid.clone())
                .with(translation(), gliderpos)
                
                .with(matter_gravity(), 9.81)
                .with(matter_local_center(), vec3(0.,0.,-1.))
                .with(buoy_radius(), 1.)
                .with(buoy_water_level(), 0.)
                .with(buoy_max_force(), 30.)
                .with(buoy_max_friction(), 5.)

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
                .with(selfie_stick(), vec3(17., 17., 25.))
                .with(selfie_focus_ent(), glider.clone())
                .with(selfie_pitch(), 0.)
                .with(selfie_yaw(), 0.)
                .spawn();

            entity::add_component(plr, plr_glidercam(), glidercam);
        }
    });
    
    query((is_glider(), translation(), glider_steer_vector(), glider_landvel(), glider_forward(), glider_forward_rotvel(), buoy_submerged())).each_frame(|gliders|{
        for (glider, (_, pos, steer_vec, landvel, fwd, rotvel, sub)) in gliders {

            let control = invlerp(0.01, 0.4, sub).clamp(0., 1.);

            let accellin = 10.0 * delta_time() * control;
            let accellerp = 0.; // 0.01 * control;
            let desired_landvel : Vec2 = steer_vec * 20.;

            // entity::set_component(glider, glider_forward(), desired_landvel.extend(1.0));
            
            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                let to_desired_landvel : Vec2 = desired_landvel - linvel.truncate();
                if to_desired_landvel.length_squared() < accellin * accellin {
                    *linvel = (
                        desired_landvel
                    ).extend(linvel.z);
                } else {
                    *linvel = (
                        linvel.xy()
                        + to_desired_landvel.clamp_length_max(accellin)
                        + to_desired_landvel * accellerp
                    ).extend(linvel.z);
                }

                entity::set_component(glider, glider_landvel(), linvel.xy());
            });

            if steer_vec.length() > 0.1 {
                let angle_to_fwd = fwd.angle_between(steer_vec);
                // let mut angle_to_fwd = fwd.angle_between(steer_vec);
                // println!("A{angle_to_fwd}");
                // if angle_to_fwd > PI * 0.8 { angle_to_fwd = -PI + angle_to_fwd; }
                // if angle_to_fwd < -PI * 0.8 { angle_to_fwd = PI + angle_to_fwd; }
                // println!("B{angle_to_fwd}");
                let desired_rotvel = (angle_to_fwd * 3.0).clamp(-PI, PI);
                let to_desired_rotvel = desired_rotvel - rotvel;
                entity::mutate_component(glider, glider_forward_rotvel(), |rotvel|*rotvel += to_desired_rotvel*0.5);
            } else {
                entity::mutate_component(glider, glider_forward_rotvel(), |rotvel|*rotvel *= 0.5); // friction i suppose?
            }
        }
    });

    query(glider_forward_rotvel()).each_frame(|gliders|{
        for(glider,fwd_rotvel) in gliders {
            entity::mutate_component(glider, glider_forward(), |forward|*forward=forward.rotate(Vec2::from_angle(fwd_rotvel*delta_time())));
        }
    });
}

fn lerp(from : f32, to : f32, rel : f32) -> f32 { ((1. - rel) * from) + (rel * to) }
fn invlerp(from : f32, to : f32, value : f32) -> f32 { (value - from) / (to - from) }