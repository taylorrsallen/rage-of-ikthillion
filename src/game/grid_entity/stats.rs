use crate::*;

//================================-================================-================================
pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<DamageEvent>()
            .add_systems((
                update_stats,
                receive_damage_events,
            ).chain().in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
pub struct DamageEvent {
    attacker: Entity,
    victim: Entity,
    multiplier: f32,
}

impl DamageEvent {
    pub fn new(
        attacker: Entity,
        victim: Entity,
        multiplier: f32,
    ) -> Self {
        Self {
            attacker,
            victim,
            multiplier,
        }
    }
}

//================================-================================-================================
#[derive(Default, Component)]
pub struct Stats {
    // active
    pub health: f32,
    pub stamina: f32,

    // primary
    pub vitality: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub luck: u32,

    // secondary
    pub level: u32,
    pub experience: f32,

    // this is retarded I know
    pub bullets: u32,
    pub reload_timer: Timer,
    pub reloading: bool,
}

impl Stats {
    pub fn new(

    ) -> Self {
        Self {
            health: 5.0,

            vitality: 5,
            endurance: 3,
            strength: 1,

            level: 5,
            experience: 0.3,

            bullets: 10,
            reload_timer: Timer::from_seconds(0.8, TimerMode::Once),
            reloading: false,
            ..default()
        }
    }
}

//================================-================================-================================

// 1  -> 0.0
// 2  -> 1.0
// 3  -> 3.0
// 4  -> 6.0
// 5  -> 10.0
// 6  -> 15.0
// 7  -> 21.0
// 8  -> 28.0
// 9  -> 36.0
// 10 -> 45.0
// 11 -> 55.0
// 12 -> 66.0
// 13 -> 78.0
// 14 -> 91.0
// 15 -> 105.0
// 16 -> 120.0
// 17 -> 136.0
// 18 -> 153.0
// (stats.level as f32 * (stats.level as f32 - 1.0)) * 0.5)

fn update_stats(
    mut commands: Commands,
    mut sound_events: EventWriter<SpatialSoundEvent>,
    mut health_query: Query<(Entity, &Transform, &mut Stats, &mut GridEntity)>,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    settings: Res<Settings>,
    player_query: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    let mut rng = thread_rng();

    let player_entity = player_query.single();

    let mut experience_to_killers: Vec<(Option<Entity>, f32, f32)> = vec![];
    
    for (entity, transform, mut stats, mut grid_entity) in health_query.iter_mut() {
        if stats.experience >= stats.level as f32 {
            stats.experience -= stats.level as f32;
            stats.level += 1;
            
            if entity == player_entity {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::LevelUp00, 1.0));
                ui_sound_events.send(UISoundEvent::new_with_delay(SoundAsset::LevelUp01, 0.6, 0.6));
            }

            for i in 0..4 {
                let level_up_stat = rng.gen_range(0..5);
                match level_up_stat {
                    0 => {
                        stats.vitality += 1;
                    },
                    1 => {
                        stats.endurance += 1;
                    },
                    2 => {
                        stats.strength += 1;
                    },
                    3 => {
                        stats.dexterity += 1;
                    },
                    4 => {
                        stats.luck += 1;
                    },
                    _ => {}
                }
            }

            stats.health += stats.vitality as f32 * 1.5;
        }

        if !grid_entity.actioning {
            let max_stamina = 30.0 + stats.vitality as f32 + (stats.endurance as f32 * 1.25);
            stats.stamina += (45.0 + (stats.endurance as f32 * 0.5)) * time.delta_seconds();
            if stats.stamina > max_stamina { stats.stamina = max_stamina; }
        }

        if stats.health <= 0.0 && !grid_entity.dying {
            grid_entity.dying = true;
            experience_to_killers.push((grid_entity.killer, stats.experience, stats.level as f32));
        }

        let mut base_regen = 0.25;
        if settings.ikthillion_unraged && entity == player_entity { base_regen = 400.0; }
        let max_health = 50.0 + (stats.vitality as f32 * 1.25) + (stats.endurance as f32 * 1.1) + stats.strength as f32;
        stats.health += (base_regen + (stats.vitality as f32 * 0.05)) * time.delta_seconds();
        if stats.health > max_health { stats.health = max_health; }

    }

    for (killer_entity, experience, level) in experience_to_killers.iter() {
        if let Some(killer_entity) = killer_entity {
            if let Ok((_, _, mut killer_stats, _)) = health_query.get_mut(*killer_entity) {
                // let mut multiplier = (level / killer_stats.level as f32); // * 0.15;
                
                // let killer_level = killer_stats.level as f32;
                // if killer_level > *level {
                //     multiplier *= level / (killer_level * ((killer_level - level) * 2.0))
                // }

                let total_experience = (experience + (level * (level - 1.0)) * 0.5); // * multiplier;
                killer_stats.experience += total_experience;
            }
        }
    }
}

fn receive_damage_events(
    mut commands: Commands,
    mut sound_events: EventWriter<SpatialSoundEvent>,
    mut damage_events: EventReader<DamageEvent>,
    mut health_query: Query<(&mut Stats, &mut PathAnim, &mut GridEntity)>,
) {
    let mut rng = thread_rng();
    for damage_event in damage_events.iter() {
        let mut attacker_weapon_def: &WeaponTypeDef;
        if let Ok((mut attacker_stats, _, attacker_grid_entity)) = health_query.get_mut(damage_event.attacker) {
            attacker_weapon_def = attacker_grid_entity.weapon.get_def();
        } else {
            println!("couldn't find attacker!");
            continue;
        }

        if let Ok((mut victim_stats, mut victim_path_anim, mut victim_grid_entity)) = health_query.get_mut(damage_event.victim) {
            victim_grid_entity.move_cd_timer.reset();
            victim_path_anim.set_path(vec![
                (victim_grid_entity.coord.as_vec3() + Vec3::Y * 0.2, 5.0),
                (victim_grid_entity.coord.as_vec3(), 5.0),
            ]);
                
            victim_stats.health -= attacker_weapon_def.damage * damage_event.multiplier;
            victim_grid_entity.killer = Some(damage_event.attacker);
        }
    }
}