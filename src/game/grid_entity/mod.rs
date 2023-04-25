use std::time::Duration;

use crate::*;

mod creature;
mod item;
mod stats;
pub use creature::*;
pub use item::*;
pub use stats::*;

use super::grid;

//================================-================================-================================
pub struct GridEntityPlugin;
impl Plugin for GridEntityPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(GridCreaturePlugin)
            .add_plugin(ItemPlugin)
            .add_plugin(StatsPlugin)
            .add_systems((
                update_grid_entity,
            ).in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct GridEntity {
    pub weapon: WeaponType,
    pub creature: CreatureType,

    pub coord: IVec3,
    pub facing: IVec3,

    pub next_action: IVec3,
    pub next_move: IVec3,

    pub wall_pressing: bool,
    pub dying_anim_started: bool,
    pub dying: bool,
    pub killer: Option<Entity>,
    pub action_anim_started: bool,
    pub actioning: bool,

    pub action_start_timer: Timer,
    pub action_recovery_timer: Timer,
    pub action_cd_timer: Timer,
    pub move_cd_timer: Timer,
    pub death_timer: Timer,
}

impl Default for GridEntity {
    fn default() -> Self {
        Self {
            weapon: WeaponType::WeakMonsterClaw,
            creature: CreatureType::EvilNinja,

            coord: IVec3::ZERO,
            facing: IVec3::Z,

            next_action: IVec3::ZERO,
            next_move: IVec3::ZERO,

            wall_pressing: false,
            dying_anim_started: false,
            dying: false,
            killer: None,
            action_anim_started: false,
            actioning: false,

            action_start_timer: Timer::from_seconds(0.08, TimerMode::Once),
            action_recovery_timer: Timer::from_seconds(0.5, TimerMode::Once),
            action_cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
            move_cd_timer: Timer::from_seconds(1.0, TimerMode::Once),
            death_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

impl GridEntity {
    pub fn new(
        weapon: WeaponType,
        creature: CreatureType,
        coord: IVec3,
        facing: IVec3,
        move_cd: f32,
    ) -> Self {
        Self {
            weapon,
            creature,

            coord,
            facing,

            next_action: IVec3::ZERO,
            next_move: IVec3::ZERO,

            wall_pressing: false,
            dying_anim_started: false,
            dying: false,
            killer: None,
            action_anim_started: false,
            actioning: false,

            action_start_timer: Timer::from_seconds(0.08, TimerMode::Once),
            action_recovery_timer: Timer::from_seconds(0.5, TimerMode::Once),
            action_cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
            move_cd_timer: Timer::from_seconds(move_cd, TimerMode::Once),
            death_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }

    pub fn set_next_action_with_direction(
        &mut self,
        direction: &Vec3,
    ) {
        self.set_next_action_with_grid_step(&GridEntity::grid_step_from_direction(direction));
    }

    pub fn set_next_action_with_grid_step(
        &mut self,
        grid_step: &IVec3,
    ) {
        self.next_action = *grid_step;
    }

    pub fn set_next_move_with_direction(
        &mut self,
        direction: &Vec3,
    ) {
        self.set_next_move_with_grid_step(&GridEntity::grid_step_from_direction(direction));
    }

    pub fn set_next_move_with_grid_step(
        &mut self,
        grid_step: &IVec3,
    ) {
        self.next_move = *grid_step;
    }

    pub fn grid_step_from_direction(
        direction: &Vec3,
    ) -> IVec3 {
        let mut grid_step = IVec3::ZERO;
        if direction.x.abs() > direction.y.abs() && direction.x.abs() > direction.z.abs() {
            if direction.x < 0.0 {
                grid_step.x = -1;
            } else {
                grid_step.x = 1;
            }
        } else if direction.y.abs() > direction.z.abs() {
            if direction.y < 0.0 {
                grid_step.y = -1;
            } else {
                grid_step.y = 1;
            }
        } else {
            if direction.z < 0.0 {
                grid_step.z = -1;
            } else {
                grid_step.z = 1;
            }
        }
    
        grid_step
    }
}

fn update_grid_entity(
    mut commands: Commands,
    mut grid_root_query: Query<&mut GridRoot>,
    mut grid_entity_query: Query<(Entity, &Transform, &GlobalTransform, &mut Stats, &mut MoveTo, &mut PathAnim, &mut GridEntity)>,
    mut spatial_sound_events: EventWriter<SpatialSoundEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<Settings>,
    mut victory_events: EventWriter<VictoryEvent>,
    mut death_events: EventWriter<DeathEvent>,
    mut bgm_events: EventWriter<BGMEvent>,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<&GlobalTransform, With<FpsCameraController>>,
    time: Res<Time>,
) {
    let camera_transform = camera_query.single();
    let mut grid_root = grid_root_query.single_mut();
    let player_entity = player_query.single();
    let mut rng = thread_rng();

    let mut hit_creatures: Vec<Entity> = vec![];
    for (entity, transform, global_transform, mut stats, mut move_to, mut path_anim, mut grid_entity) in grid_entity_query.iter_mut() {
        if global_transform.translation().distance(camera_transform.translation()) > DRAW_DISTANCE {
            continue;
        }

        if grid_entity.dying {
            if entity == player_entity && !settings.player_died {
                bgm_events.send(BGMEvent::new(SoundAsset::Death, 1.0, BGMType::Music));
                death_events.send(DeathEvent);
            }

            if !grid_entity.dying_anim_started {
                if grid_entity.creature.get_def().aggressive {
                    settings.evil_vanquished += 1;
                    if grid_entity.creature == CreatureType::EvilSkeleton {
                        bgm_events.send(BGMEvent::new(SoundAsset::SavingAllIkthia, 1.0, BGMType::Music));
                        victory_events.send(VictoryEvent);
                    }
                } else {
                    settings.innocents_murdered += 1;
                }

                grid_entity.dying_anim_started = true;
                grid_root.set_creature_at_coord(&grid_entity.coord, &None);
                
                move_to.look_axis = grid_entity.facing.as_vec3();
                const RAND_LOOK_DIRECTION: [Vec3; 2] = [Vec3::Y, Vec3::NEG_Y];
                move_to.look_direction = RAND_LOOK_DIRECTION[rng.gen_range(0..2)];
                move_to.position = grid_entity.coord.as_vec3() - Vec3::Y * 0.45;
                path_anim.clear();
                spatial_sound_events.send(SpatialSoundEvent::new(grid_entity.creature.get_def().get_death_sound(), 0.4, grid_entity.coord.as_vec3()));
            }
            
            grid_entity.death_timer.tick(time.delta());
            if grid_entity.death_timer.just_finished() {
                if entity != player_entity {
                    commands.entity(entity).despawn_recursive();
                    spatial_sound_events.send(SpatialSoundEvent::new(POPS[rng.gen_range(0..POPS.len())], 0.4, grid_entity.coord.as_vec3()));
                }

            }

            continue;
        }

        move_to.look_direction = grid_entity.next_move.as_vec3();
        move_to.speed = 5.0 + stats.dexterity as f32 * 0.05;

        let weapon_def = grid_entity.weapon.get_def();
        if grid_entity.actioning {
            grid_entity.action_start_timer.tick(Duration::from_secs_f32((time.delta_seconds() + (weapon_def.attack_delay * 0.0005 + stats.dexterity as f32 * 0.0005)).min(1.0)));
            if grid_entity.action_start_timer.just_finished() {
                let origin_coord = grid_entity.coord;
                let action_tile_coord = origin_coord + grid_entity.next_action;
                let action_tile = grid_root.get_tile_from_coord(&action_tile_coord);

                if action_tile.is_solid() {
                    // hit a wall
                    if grid_entity.weapon == WeaponType::Pickaxe {
                        spatial_sound_events.send(SpatialSoundEvent::new(STONE_BREAKS[rng.gen_range(0..STONE_BREAKS.len())], 1.3, action_tile_coord.as_vec3()));
                        grid_root.set_tile_on_at_coord(&action_tile_coord, &GridTile::open());
                    } else {
                        spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::HitSwordMetal00, 0.1, action_tile_coord.as_vec3()));
                    }
                }

                if let Some(action_tile_creature) = grid_root.get_creature_from_coord(&action_tile_coord) {
                    // hit a creature
                    hit_creatures.push(*action_tile_creature);
                    if rng.gen::<f32>() < weapon_def.crit_chance + (stats.luck as f32 * 0.0005) + (stats.dexterity as f32 * 0.00025) {
                        if grid_entity.weapon == WeaponType::Katana { stats.health += 25.0; }
                        damage_events.send(DamageEvent::new(entity, *action_tile_creature, weapon_def.crit_power + (stats.luck as f32 * 0.025) + (stats.dexterity as f32 * 0.05) + (stats.strength as f32 * 0.1)));
                        spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::HitCritical00, 0.3, action_tile_coord.as_vec3()));
                    } else {
                        if grid_entity.weapon == WeaponType::Katana { stats.health += 10.0; }
                        damage_events.send(DamageEvent::new(entity, *action_tile_creature, 1.0 + (stats.strength as f32 * 0.1) + (stats.luck as f32 * 0.005)));
                        spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::Hit01, 0.4, action_tile_coord.as_vec3()));
                    }
                } else if grid_entity.weapon == WeaponType::HolyHandgunOfAntinuk {
                    for z in 0..20 {
                        let bullet_tile = action_tile_coord + grid_entity.next_action * z;
                        if let Some(action_tile_creature) = grid_root.get_creature_from_coord(&bullet_tile) {
                            spatial_sound_events.send(SpatialSoundEvent::new(grid_entity.creature.get_def().get_hurt_sound(), 0.3, action_tile_coord.as_vec3()));
                            if rng.gen::<f32>() < weapon_def.crit_chance {
                                damage_events.send(DamageEvent::new(entity, *action_tile_creature, weapon_def.crit_power));
                                spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::HitCritical00, 0.3, action_tile_coord.as_vec3()));
                            } else {
                                damage_events.send(DamageEvent::new(entity, *action_tile_creature, 1.0));
                                spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::Hit01, 0.4, action_tile_coord.as_vec3()));
                            }

                            break;
                        }
                    }
                }
            } else {
                grid_entity.action_recovery_timer.tick(Duration::from_secs_f32((time.delta_seconds() + (weapon_def.attack_recovery * 0.0005 + stats.dexterity as f32 * 0.0005)).min(1.0)));
                if grid_entity.action_recovery_timer.just_finished() {
                    grid_entity.actioning = false;
                    grid_entity.action_anim_started = false;
                    grid_entity.next_action = IVec3::ZERO;
                    grid_entity.action_cd_timer.reset();
                }
            }
        } else {
            let dexterity_value = stats.dexterity as f32 * 0.0005;
            grid_entity.action_cd_timer.tick(Duration::from_secs_f32((time.delta_seconds() + (dexterity_value)).min(1.0)));
            if grid_entity.next_action != IVec3::ZERO && grid_entity.action_cd_timer.finished() {
                let stamina_use = grid_entity.weapon.get_def().stamina_use * (1.0 - ((stats.endurance as f32 * 0.005) + stats.strength as f32 * 0.01));
                if grid_entity.weapon == WeaponType::HolyHandgunOfAntinuk {
                    if stats.bullets > 0 {
                        stats.bullets -= 1;
                    } else if !stats.reloading {
                        spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::HolyHandgunOfAntinuk03, 0.8, grid_entity.coord.as_vec3()));
                        stats.reloading = true;
                        stats.reload_timer.reset();
                    }

                    if stats.reloading {
                        stats.reload_timer.tick(Duration::from_secs_f32((time.delta_seconds() + (dexterity_value)).min(1.0)));
                        if stats.reload_timer.finished() {
                            stats.reloading = false;
                            stats.bullets = 10;
                        }
                    } 
                }
                
                if stats.stamina >  0.0 {
                    if grid_entity.weapon == WeaponType::HolyHandgunOfAntinuk && stats.reloading {

                    } else {
                        stats.stamina -= stamina_use;
                        let origin_coord = grid_entity.coord;
                        let action_tile_coord = origin_coord + grid_entity.next_action;
                        let action_tile = grid_root.get_tile_from_coord(&action_tile_coord);
            
                        grid_entity.actioning = true;
    
                        spatial_sound_events.send(SpatialSoundEvent::new(weapon_def.attack_sound, 0.5, action_tile_coord.as_vec3()));
                        grid_entity.action_start_timer.reset();
                        grid_entity.action_recovery_timer.reset();
                    }
                }
            }
        }
        
        
        grid_entity.move_cd_timer.tick(Duration::from_secs_f32((time.delta_seconds() + (stats.dexterity as f32 * 0.00005)).min(1.0)));
        if grid_entity.move_cd_timer.finished() {
            if grid_entity.next_move != IVec3::ZERO {
                let start_coord = grid_entity.coord;
                let step_tile_coord = start_coord + grid_entity.next_move;
                let step_tile = grid_root.get_tile_from_coord(&step_tile_coord);
                
                if step_tile.is_solid() {
                    if !grid_entity.wall_pressing {
                        grid_entity.wall_pressing = true;
                        let wall_press_position = start_coord.as_vec3() + grid_entity.next_move.as_vec3() * 0.15;
                        move_to.position = wall_press_position;
                        spatial_sound_events.send(SpatialSoundEvent::new(SoundAsset::Cloth02, 1.2, wall_press_position));
                    }
                    
                    continue;
                }
                
                let step_floor_tile_coord = step_tile_coord - IVec3::Y;
                let step_floor_tile = grid_root.get_tile_from_coord(&step_floor_tile_coord);
                
                if let Some(step_tile_creature) = grid_root.get_creature_from_coord(&step_tile_coord) {
                    // walking into a creature
                    
                } else if step_floor_tile.is_solid() {
                    // there is no entity and the floor is walkable
                    grid_entity.coord = step_tile_coord;
                    grid_entity.next_move = IVec3::ZERO;
                    path_anim.clear();
                    move_to.position = grid_entity.coord.as_vec3();
                    
                    spatial_sound_events.send(SpatialSoundEvent::new(step_floor_tile.get_step_sound(), 1.0, grid_entity.coord.as_vec3()));
                    
                    grid_entity.move_cd_timer.reset();

                    grid_root.set_creature_at_coord(&start_coord, &None);
                    grid_root.set_creature_at_coord(&grid_entity.coord, &Some(entity));
                }
            } else {
                move_to.position = grid_entity.coord.as_vec3();
                grid_entity.wall_pressing = false;
            }
        }
    }

    for hit_creature in hit_creatures.iter() {
        if let Ok((_, _, _, _, _, _, hit_grid_entity)) = grid_entity_query.get(*hit_creature) {
            spatial_sound_events.send(SpatialSoundEvent::new(hit_grid_entity.creature.get_def().get_hurt_sound(), 0.3, hit_grid_entity.coord.as_vec3()));
        }
    }
}