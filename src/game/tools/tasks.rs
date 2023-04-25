use bevy::tasks::ThreadExecutor;
use rand::{thread_rng, Rng};

use crate::*;

//================================-================================-================================
pub struct TasksPlugin;
impl Plugin for TasksPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Tasks::default())
            .add_systems((
                    perform_tasks,
                    update_entity_scenes,
                ).in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
pub struct CreatureSpawnTask {
    coord: IVec3,
    facing: IVec3,
    creature_type: CreatureType,
}

#[derive(Resource)]
pub struct Tasks {
    creature_spawn_tasks: Vec<CreatureSpawnTask>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            creature_spawn_tasks: vec![],
        }
    }
}

impl Tasks {
    pub fn spawn_creature(
        &mut self,
        coord: &IVec3,
        facing: &IVec3,
        creature_type: CreatureType,
    ) {
        self.creature_spawn_tasks.push(CreatureSpawnTask {coord: *coord, facing: *facing, creature_type});
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct EntityScene;
#[derive(Component)]
pub struct EntitySceneOwner {
    current_scene: Option<Entity>,
    scene_asset: SceneAsset,
}

fn perform_tasks(
    mut commands: Commands,
    mut grid_root_query: Query<&mut GridRoot>,
    mut tasks: ResMut<Tasks>,
    asset_map: Res<AssetMap>,
) {
    let mut grid_root = grid_root_query.single_mut();
    let mut rng = thread_rng();
    loop {
        if let Some(creature_spawn_task) = tasks.creature_spawn_tasks.pop() {
            let creature_def = creature_spawn_task.creature_type.get_def();
            let weapon_def = creature_def.weapon.get_def();
            let creature = commands.spawn(Creature::new(creature_def.aggressive))
                .insert(GridEntity {
                    weapon: WeaponType::WeakMonsterClaw,
                    creature: creature_spawn_task.creature_type,
                    coord: creature_spawn_task.coord,
                    facing: creature_spawn_task.facing,
                    move_cd_timer: Timer::from_seconds(creature_def.move_cd, TimerMode::Once),
                    ..default()
                })
                .insert(Stats {
                    health: f32::MAX,
                    stamina: f32::MAX,

                    vitality: creature_def.base_vitality,
                    endurance: creature_def.base_endurance,
                    strength: creature_def.base_strength,
                    dexterity: creature_def.base_dexterity,
                    luck: creature_def.base_luck,
                    level: creature_def.base_level,
                    experience: creature_def.base_experience,

                    bullets: 0,
                    reload_timer: Timer::from_seconds(0.8, TimerMode::Once),
                    reloading: false,
                })
                .insert(MoveTo::new(&creature_spawn_task.facing.as_vec3(), &creature_spawn_task.coord.as_vec3(), 5.0))
                .insert(PathAnim::new(vec![]))
                .insert(TransformBundle {
                    local: Transform::from_translation(creature_spawn_task.coord.as_vec3()),
                    ..default()
                })
                .insert(VisibilityBundle::default())
                .insert(EntitySceneOwner { current_scene: None, scene_asset: creature_def.scene })
                .insert(Name::new(creature_def.name))
                .id();

            if creature_spawn_task.creature_type == CreatureType::EvilSkeleton {
                commands.entity(creature).insert(IkthillionTracker);
            }

            grid_root.set_creature_at_coord(&creature_spawn_task.coord, &Some(creature))
        } else {
            break;
        }
    }
}

fn update_entity_scenes(
    mut commands: Commands,
    mut entity_scene_owner_query: Query<(Entity, &Transform, &mut EntitySceneOwner)>,
    entity_scene_query: Query<Entity, With<EntityScene>>,
    camera_query: Query<&Transform, With<Camera3d>>,
    asset_map: Res<AssetMap>,
) {
    let camera_transform = camera_query.single();

    for (owner_entity, owner_transform, mut owner) in entity_scene_owner_query.iter_mut() {
        if camera_transform.translation.distance(owner_transform.translation) > DRAW_DISTANCE {
            if let Some(scene_entity) = owner.current_scene {
                commands.entity(scene_entity).despawn_recursive();
                owner.current_scene = None;
            }
        } else {
            if owner.current_scene == None {
                let scene_entity = commands.spawn(SceneBundle {
                    scene: asset_map.get_scene_handle(owner.scene_asset),
                    transform: Transform::from_translation(Vec3::NEG_Y * 0.5).with_scale(Vec3::ONE * 0.5),
                    ..default()
                })
                .insert(EntityScene)
                .id();

                owner.current_scene = Some(scene_entity);
                commands.entity(owner_entity).push_children(&[scene_entity]);
            }
        }
    }
}