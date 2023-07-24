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
use crate::components::{glider_landvel, glider_desired_landvel, glider_hook_pos};
use crate::components::{selfie_stick, selfie_focus_ent, selfie_pitch, selfie_yaw};

pub fn setup() {
    spawn_query((player(), user_id())).bind(|players|{
        for (plr, (_,uid)) in players {

            // random land position, but on the ground
            let gliderpos = vec3(
                random::<f32>()*1.,
                random::<f32>()*5. + 10.,

                random::<f32>()*1. + 1., // drop from a slight height
            );

            let glider = Entity::new()
                .with_merge(make_transformable())
                .with(physics_controlled(), ())
                .with(dynamic(), true)
                // .with(cube_collider(), vec3(0.5, 0.5, 0.5))
                .with(sphere_collider(), 0.660/2.) // fills 66% of the corridor
                .with(visualize_collider(), ())
                .with(linear_velocity(), Vec3::ZERO)
                .with(angular_velocity(), Vec3::ZERO)

                .with(name(), "Hook pos".to_string())
                .with(is_glider(), ())
                .with(glider_landvel(), vec2(0., -1.))
                .with(glider_desired_landvel(), vec2(0., -1.))
                .with(glider_hook_pos(), gliderpos)
                .with(user_id(), uid.clone())
                // .with(cube(), ()) // hidden. see c_playeranim.
                .with(translation(), gliderpos)
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
            entity::mutate_component(glider, glider_landvel(), move |landvel|{
                *landvel = vel.truncate();
                *landvel *= 1.-friction;
                let to_desired_landvel : Vec2 = desired_landvel - *landvel;
                if to_desired_landvel.length_squared() < accellin * accellin {
                    *landvel = desired_landvel;
                } else {
                    *landvel += to_desired_landvel.clamp_length_max(accellin) + to_desired_landvel * accellerp;
                }
                entity::set_component(glider, linear_velocity(), landvel.extend(vel.z));
            });
            entity::mutate_component(glider, linear_velocity(), move |linvel|{
                linvel.z -= 9.81 * delta_time(); // gravity
                if pos.z - 1.0 < 0. {
                    linvel.z *= 0.95; // water has drag
                    linvel.z += -(pos.z - 1.0) * 20. * delta_time(); // buoyancy
                }
            });
        }
    });

    // try to stay upright somehow

    query((is_glider(), rotation(), angular_velocity())).each_frame(|gliders|{
        for (glider, (_, rot, _)) in gliders {
            entity::mutate_component(glider, angular_velocity(), move |angvel|{
                *angvel *= 0.50; // always reduce angvel
            });
        }
    });
}