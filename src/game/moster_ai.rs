use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use crate::game::living_being::{LivingBeingDeathEvent, LivingBeingHitEvent};

use super::super::AppState;
use super::{GameDirection, Jumper, Bug};

struct MonsterWalkedIntoWallEvent {
    entity: Entity,
}

pub struct MonsterAiPlugin;

impl Plugin for MonsterAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterWalkedIntoWallEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(monster_walking_system)
                    .with_system(monster_wall_contact_detection)
                    .with_system(monster_change_direction_on_contact),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(2.0))
                    .with_system(monster_jumps),
            ).add_event::<LivingBeingHitEvent>()
            .add_event::<LivingBeingDeathEvent>();
    }
}

fn monster_walking_system(mut monsters: Query<(&Bug, &mut Velocity)>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        let speed = match monster.facing_direction {
            GameDirection::Left => -monster.speed,
            GameDirection::Right => monster.speed,
        };

        velocity.linvel = Vec2::new(speed, velocity.linvel.y);
    }
}

fn monster_wall_contact_detection(
    monsters: Query<Entity, With<Bug>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut send_monster_walked_into_wall: EventWriter<MonsterWalkedIntoWallEvent>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _) = collision_event {
            for monster in monsters.iter() {
                if *h1 == monster || *h2 == monster {
                    send_monster_walked_into_wall
                        .send(MonsterWalkedIntoWallEvent { entity: monster })
                }
            }
        }
    }
}

fn monster_change_direction_on_contact(
    mut events: EventReader<MonsterWalkedIntoWallEvent>,
    mut monster_query: Query<&mut Bug>,
) {
    for event in events.iter() {
        // bullet contacts may destroy monster before running this system.
        if let Ok(mut monster) = monster_query.get_mut(event.entity) {
            monster.facing_direction = match monster.facing_direction {
                GameDirection::Left => GameDirection::Right,
                GameDirection::Right => GameDirection::Left,
            }
        }
    }
}

fn monster_jumps(mut monsters: Query<(&mut Jumper, &mut Velocity), With<Bug>>) {
    for (monster, mut velocity) in monsters.iter_mut() {
        if should_jump() {
            velocity.linvel = Vec2::new(0., monster.jump_impulse);
        }
    }
}

fn should_jump() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(0.1)
}
