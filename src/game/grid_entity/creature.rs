use crate::*;
use rand::Rng;

//================================-================================-================================
const GRID_STEPS: [IVec3; 4] = [
    IVec3::new(-1,  0,  0),
    IVec3::new( 1,  0,  0),
    IVec3::new( 0,  0, -1),
    IVec3::new( 0,  0,  1),
];

//================================-================================-================================
pub struct GridCreaturePlugin;
impl Plugin for GridCreaturePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems((
                move_creatures_randomly,
            ).in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct Creature {
    pub aggressive: bool,
}

impl Creature {
    pub fn new(
        aggressive: bool,
    ) -> Self {
        Self {
            aggressive,
        }
    }
}

pub fn move_creatures_randomly(
    mut grid_root_query: Query<&mut GridRoot>,
    mut grid_creature_query: Query<(&mut GridEntity, &Transform, &mut Creature), (Without<Player>, Without<FpsCameraController>)>,
    player_query: Query<(Entity, &GridEntity), With<Player>>,
    camera_query: Query<&GlobalTransform, With<FpsCameraController>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let mut root = grid_root_query.single_mut();

    if let Ok((player_entity, player_grid_entity)) = player_query.get_single() {
        let camera_transform = camera_query.single();
        'main: for (mut grid_entity, transform, mut creature) in grid_creature_query.iter_mut() {
            if camera_transform.translation().distance(transform.translation) > DRAW_DISTANCE {
                continue;
            }
    
            if creature.aggressive {
                let current_step_coord = grid_entity.coord + grid_entity.next_move;
                let current_step_tile = root.get_tile_from_coord(&current_step_coord);
                let current_step_creature = root.get_creature_from_coord(&current_step_coord);

                if grid_entity.next_move != IVec3::ZERO {
                    if !current_step_tile.is_solid() {
                        if let Some(current_step_creature) = current_step_creature {
                            if *current_step_creature == player_entity {
                                let grid_step = grid_entity.next_move;
                                grid_entity.set_next_action_with_grid_step(&grid_step);
                                continue;
                            }
                        }
                    }
                } else {
                    let mut valid_steps: Vec<IVec3> = vec![];
                    for step in GRID_STEPS {
                        let step_coord = grid_entity.coord + step;
                        let step_tile = root.get_tile_from_coord(&step_coord);
                        let step_creature = root.get_creature_from_coord(&step_coord);

                        if !step_tile.is_solid() {
                            if step_creature.is_some() {
                                if step_creature.unwrap() == player_entity {
                                    grid_entity.set_next_action_with_grid_step(&step);
                                    grid_entity.set_next_move_with_grid_step(&step);
                                    continue 'main;
                                }
                            } else if step_creature.is_none() {
                                valid_steps.push(step);
                            }
                        }
                    }

                    if !valid_steps.is_empty() {
                        let mut best_step = (IVec3::ZERO, f32::MAX);
                        for step in valid_steps.iter() {
                            let distance_from_player = (*step + grid_entity.coord).as_vec3().distance(player_grid_entity.coord.as_vec3());
                            if distance_from_player < best_step.1 {
                                best_step.0 = *step;
                                best_step.1 = distance_from_player;
                            }
                        }

                        grid_entity.set_next_move_with_grid_step(&best_step.0);
                    }
                }
            } else {
                let current_step_coord = grid_entity.coord + grid_entity.next_move;
                let current_step_tile = root.get_tile_from_coord(&current_step_coord);
                let current_step_creature = root.get_creature_from_coord(&current_step_coord);
    
                if grid_entity.next_move != IVec3::ZERO {
                    if current_step_creature.is_none() && !current_step_tile.is_solid() {
                        continue;
                    } else {
                        grid_entity.next_move = IVec3::ZERO;
                    }
                } else {
                    let mut valid_steps: Vec<IVec3> = vec![];
                    for step in GRID_STEPS {
                        let step_coord = grid_entity.coord + step;
                        let step_tile = root.get_tile_from_coord(&step_coord);
                        let step_creature = root.get_creature_from_coord(&step_coord);
                        
                        if !step_tile.is_solid() && step_creature.is_none() {
                                valid_steps.push(step);
                        }
                    }
                    
                    if !valid_steps.is_empty() {
                        let grid_step = valid_steps[rng.gen_range(0..valid_steps.len())];
                        grid_entity.set_next_move_with_grid_step(&grid_step);
                        grid_entity.facing = grid_step;
                    }
                }
            }
        }
    }
}