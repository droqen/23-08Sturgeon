use ambient_api::{
    components::core::{
        app::{main_scene,name,},
        camera::aspect_ratio_from_window,
        model::model_from_url,
        player::{player,user_id},
        // primitives::cube,
        physics::{
            // cube_collider,
            // sphere_collider,
            physics_controlled, dynamic, linear_velocity, angular_velocity, visualize_collider,
            collider_from_url,
        },
        transform::{lookat_target, translation, rotation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
    // glam::EulerRot,
};

use crate::components::{is_glider, is_glidercam};
use crate::components::{plr_glider, plr_glidercam};
use crate::components::{glider_landvel, glider_desired_landvel, glider_hook_pos};
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
                // .with(sphere_collider(), 0.70)
                .with(model_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
                .with(collider_from_url(), asset::url("assets/MSH_Boat.glb").unwrap())
                .with(visualize_collider(), ())

                .with(linear_velocity(), vec3(0., 0., 3.)) // toss up
                .with(angular_velocity(), Vec3::ZERO)

                .with(name(), "Hook pos".to_string())
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(glider_desired_landvel(), vec2(0., -1.))
                .with(glider_hook_pos(), gliderpos.truncate().extend(0.))
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
    
    query((is_glider(), translation(), glider_desired_landvel(), glider_landvel(), linear_velocity())).each_frame(|gliders|{
        for (glider, (_, pos, desired_landvel, landvel, vel)) in gliders {
            let accellin = 0.5 * delta_time();
            let accellerp = 0.02;
            let friction = 0.01;

            let mut achievable_landvel = desired_landvel;
            if let Some(submerged) = entity::get_component(glider, buoy_submerged()) {
                if submerged < 0.50 {
                    achievable_landvel *= 0.25 + 0.75 * submerged * 2.;
                }
            } else {
                achievable_landvel *= 0.00; // no movement is allowed? maybe allow twitches or something
            }
            
            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                *linvel *= 1.-friction;
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


            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                linvel.z -= 9.81 * delta_time(); // gravity
                if pos.z - 1.0 < 0. {
                    linvel.z *= 0.95; // water has drag
                    linvel.z += -(pos.z - 1.0) * delta_time() * (15. + 10. * random::<f32>()); // buoyancy slightly unpredictable
                }
            });
        }
    });
}