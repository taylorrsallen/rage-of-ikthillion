use crate::*;
pub use bevy::{
    utils::HashMap,
    render::{
        mesh::*,
        render_resource::*,
    },
};

mod grid_root;
mod grid_chunk;
mod grid_tile;
mod proc_mesh;
pub use grid_root::*;
pub use grid_chunk::*;
pub use grid_tile::*;
pub use proc_mesh::*;


//================================-================================-================================
pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_grid_root.in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
pub const CHUNK_LOG2DIM: usize = 2;

pub const CHUNK_DIM: usize =       1 << CHUNK_LOG2DIM;
pub const CHUNK_SIZE: usize =      1 << (CHUNK_LOG2DIM*3);
pub const CHUNK_MASK_SIZE: usize = CHUNK_SIZE >> 6;
pub const CHUNK_LEVEL: usize =     0;
pub const CHUNK_ORIGIN_MASK: i32 = !(CHUNK_DIM as i32 - 1);

pub const DRAW_DISTANCE: f32 = 10.0;

//================================-================================-================================
fn update_grid_root(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid_root_query: Query<&mut GridRoot, Changed<GridRoot>>,
    player_query: Query<&Transform, With<Player>>,
    asset_map: Res<AssetMap>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut new_chunk_meshes: Vec<(IVec3, Entity)> = vec![];
        let mut despawn_chunk_meshes: Vec<(IVec3)> = vec![];
    
        if let Ok(root) = grid_root_query.get_single() {
            for (key, chunk) in root.chunks.iter() {
                if key.as_vec3().distance(player_transform.translation) > DRAW_DISTANCE {
                    if let Some(mesh_entity) = chunk.mesh_entity {
                        despawn_chunk_meshes.push(*key);
                    }
                } else {
                    let (mesh, scenes) = chunk.get_mesh_and_scenes(root, &mut commands, &asset_map);
                    let new_mesh_entity = commands.spawn(PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(StandardMaterial {
                                base_color: Color::WHITE,
                                base_color_texture: Some(asset_map.texture_atlas.clone()),
                                perceptual_roughness: 1.0,
                                ..default()
                            }),
                            transform: Transform::from_translation(chunk.origin.as_vec3()),
                            ..default()
                        })
                        .push_children(&scenes)
                        .insert(Name::new("Chunk Mesh"))
                        .id();
        
                    new_chunk_meshes.push((*key, new_mesh_entity));
                }
            }
        }
    
        if let Ok(mut root) = grid_root_query.get_single_mut() {
            for key in despawn_chunk_meshes.iter() {
                if let Some(mut chunk) = root.chunks.get_mut(key) {
                    commands.entity(chunk.mesh_entity.unwrap()).despawn_recursive();
                    chunk.mesh_entity = None;
                }
            }

            for (key, new_mesh_entity) in new_chunk_meshes.iter() {
                if let Some(mut chunk) = root.chunks.get_mut(key) {
                    if let Some(old_mesh_entity) = chunk.mesh_entity {
                        commands.entity(old_mesh_entity).despawn_recursive();
                    }
    
                    chunk.mesh_entity = Some(*new_mesh_entity);
                }
            }
        }
    }
}