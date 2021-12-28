use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::{
    FromPlayer, Laser, Materials, Player, PlayerReadyFire, PlayerState, Speed, WinSize, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerState::default())
            .add_startup_stage(
                "game_setup_actors",
                SystemStage::single(player_spawn.system()),
            )
            .add_system(player_movement.system())
            .add_system(player_fire.system())
            .add_system(laser_movement.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn.system()),
            );
    }
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dir = if input.pressed(KeyCode::Left) {
            -1.
        } else if input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(&mut Transform, &mut PlayerReadyFire, With<Player>)>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player_tf, mut ready_fire, _)) = query.single_mut() {
        if ready_fire.0 && input.pressed(KeyCode::Space) {
            ready_fire.0 = false;
            let x = player_tf.translation.x;
            let y = player_tf.translation.y;

            let mut spawn_lasers = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.1),
                            scale: Vec3::new(0.5, 0.5, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Laser)
                    .insert(FromPlayer)
                    .insert(Speed::default());
            };
            let x_offset = 144. / 4. - 5.0;
            spawn_lasers(x_offset);
            spawn_lasers(-x_offset);
        }

        if input.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromPlayer>)>,
) {
    for (laser_entity, speed, mut transform) in query.iter_mut() {
        transform.translation.y += speed.0 * TIME_STEP;
        if transform.translation.y > win_size.h {
            commands.entity(laser_entity).despawn();
        }
    }
}

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
    time: Res<Time>,
    mut player_state: ResMut<PlayerState>,
) {
    let now = time.seconds_since_startup();
    let last_shot = player_state.last_shot;

    // spawn a sprite
    if !player_state.on && (last_shot == 0. || now > last_shot + 2.) {
        let bottom = -win_size.h / 2.;
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                    scale: Vec3::new(0.5, 0.5, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player)
            .insert(PlayerReadyFire(true))
            .insert(Speed::default());

        player_state.spawned();
    }
}