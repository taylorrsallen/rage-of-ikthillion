use crate::*;

//================================-================================-================================
#[derive(Clone)]
pub struct GridChunk {
    pub origin: IVec3,
    pub mesh_entity: Option<Entity>,

    pub tiles: [GridTile; CHUNK_SIZE],
    pub creatures: [Option<Entity>; CHUNK_SIZE],
    pub items: [Option<Entity>; CHUNK_SIZE],
    
    pub tile_mask: Bitmask,
    pub creature_mask: Bitmask,
    pub item_mask: Bitmask,
    pub dirty_mask: Bitmask,
    
    pub new: bool,
    pub dirty: bool,
    pub background: GridTile,
}

impl Default for GridChunk {
    fn default() -> Self {
        Self {
            origin: IVec3::new(0, 0, 0),
            mesh_entity: None,
            
            tiles: [GridTile::default(); CHUNK_SIZE],
            creatures: [None; CHUNK_SIZE],
            items: [None; CHUNK_SIZE],
            
            tile_mask: Bitmask::from_cube_log2dim(CHUNK_LOG2DIM, false),
            creature_mask: Bitmask::from_cube_log2dim(CHUNK_LOG2DIM, false),
            item_mask: Bitmask::from_cube_log2dim(CHUNK_LOG2DIM, false),
            dirty_mask: Bitmask::from_cube_log2dim(CHUNK_LOG2DIM, false),
            
            new: false,
            dirty: false,
            background: GridTile::default(),
        }
    }
}

impl GridChunk {
    pub fn new(
        coord: &IVec3,
        background: GridTile,
    ) -> Self {
        Self {
            origin: *coord & CHUNK_ORIGIN_MASK,
            
            tiles: [background; CHUNK_SIZE],
            
            background,
            ..default()
        }
    }
    
    pub fn index_from_coord(
        coord: &IVec3
    ) -> usize {
        (((coord.x & (CHUNK_DIM as i32 - 1)) << 2*CHUNK_LOG2DIM) +
        ((coord.y & (CHUNK_DIM as i32 - 1)) <<   CHUNK_LOG2DIM) +
        (coord.z & (CHUNK_DIM as i32 - 1))) as usize
    }
    
    pub fn local_coord_from_index(
        index: usize,
    ) -> IVec3 {
        let x = (index >> (2*CHUNK_LOG2DIM)) as i32;
        let n = index & ((1 << 2*CHUNK_LOG2DIM)-1);
        let y = (n >> CHUNK_LOG2DIM) as i32;
        let z = (n & ((1 << CHUNK_LOG2DIM)-1)) as i32;
        
        IVec3::new(x, y, z)
    }

    pub fn global_coord_from_index(
        &self,
        index: usize,
    ) -> IVec3 {
        GridChunk::local_coord_from_index(index) + self.origin
    }

    

    pub fn get_tile_from_index(
        &self,
        index: usize,
    ) -> &GridTile {
        &self.tiles[index]
    }

    pub fn get_tile_from_coord(
        &self,
        coord: &IVec3,
    ) -> &GridTile {
        self.get_tile_from_index(GridChunk::index_from_coord(coord))
    }

    pub fn get_mut_tile_from_index(
        &mut self,
        index: usize,
    ) -> &mut GridTile {
        &mut self.tiles[index]
    }

    pub fn get_mut_tile_from_coord(
        &mut self,
        coord: &IVec3,
    ) -> &mut GridTile {
        self.get_mut_tile_from_index(GridChunk::index_from_coord(coord))
    }

    pub fn set_tile_on_at_index(
        &mut self,
        index: usize,
        tile: &GridTile,
    ) {
        self.tiles[index] = *tile;
        self.tile_mask.set_bit_on(index);
        self.dirty_mask.set_bit_on(index);
        self.dirty = true;
    }
    
    pub fn set_tile_off_at_index(
        &mut self,
        index: usize,
    ) {
        self.tiles[index] = self.background;
        self.tile_mask.set_bit_off(index);
        self.dirty_mask.set_bit_off(index);
        self.dirty = true;
    }

    pub fn set_tile_on_at_coord(
        &mut self,
        coord: &IVec3,
        tile: &GridTile,
    ) {
        self.set_tile_on_at_index(GridChunk::index_from_coord(coord), tile);
    }
    
    pub fn set_tile_off_at_coord(
        &mut self,
        coord: &IVec3,
    ) {
        self.set_tile_off_at_index(GridChunk::index_from_coord(coord));
    }



    pub fn get_creature_from_index(
        &self,
        index: usize,
    ) -> &Option<Entity> {
        &self.creatures[index]
    }

    pub fn get_creature_from_coord(
        &self,
        coord: &IVec3,
    ) -> &Option<Entity> {
        self.get_creature_from_index(GridChunk::index_from_coord(coord))
    }

    pub fn set_creature_at_index(
        &mut self,
        index: usize,
        entity: &Option<Entity>,
    ) {
        self.creatures[index] = *entity;
        self.creature_mask.set_bit_on(index);
        self.dirty_mask.set_bit_on(index);
    }

    pub fn set_creature_at_coord(
        &mut self,
        coord: &IVec3,
        entity: &Option<Entity>,
    ) {
        self.set_creature_at_index(GridChunk::index_from_coord(coord), entity);
    }



    pub fn get_item_from_index(
        &self,
        index: usize,
    ) -> &Option<Entity> {
        &self.items[index]
    }

    pub fn get_item_from_coord(
        &self,
        coord: &IVec3,
    ) -> &Option<Entity> {
        self.get_item_from_index(GridChunk::index_from_coord(coord))
    }

    pub fn set_item_at_index(
        &mut self,
        index: usize,
        entity: &Option<Entity>,
    ) {
        self.items[index] = *entity;
        self.item_mask.set_bit_on(index);
        self.dirty_mask.set_bit_on(index);
    }

    pub fn set_item_at_coord(
        &mut self,
        coord: &IVec3,
        entity: &Option<Entity>,
    ) {
        self.set_item_at_index(GridChunk::index_from_coord(coord), entity);
    }



    pub fn get_mesh_and_scenes(
        &self,
        grid_root: &GridRoot,
        commands: &mut Commands,
        asset_map: &Res<AssetMap>,
    ) -> (Mesh, Vec<Entity>) {
        let mut mesh_data = MeshData::default();
        let mut scenes: Vec<Entity> = vec![];

        for tile_index in OnMaskIter::new(0, &self.tile_mask) {
            let tile = &self.tiles[tile_index];
            let local_coord = Self::local_coord_from_index(tile_index);

            if tile.tile_type != TileType::Block {
                mesh_data.cut_cube_from_mesh(tile.matter, &local_coord, &(local_coord + self.origin), grid_root);
                
                if tile.tile_type != TileType::Open {
                    let tile_scene = commands.spawn(TransformBundle {
                        local: Transform::from_translation(local_coord.as_vec3()),
                        ..default()
                    })
                    .insert(VisibilityBundle::default())
                    // .with_children(|child_builder| {
                    //     child_builder.spawn(SceneBundle {
                    //             scene: asset_map.get_scene_handle(tile.get_scene()),
                    //             transform: Transform::from_translation(Vec3::NEG_Y * 0.5).with_scale(Vec3::ONE * 0.5),
                    //             ..default()
                    //         });
                    // })
                    .id();
        
                    scenes.push(tile_scene);
                }
            } else {
                // mesh_data.add_cube_to_mesh(tile.matter, &local_coord, &(local_coord + self.origin), grid_root);
            }
        }

        (mesh_data.get_mesh(), scenes)
    }

    pub fn despawn_tile_scenes(
        &mut self,
        commands: &mut Commands,
    ) {
        
    }
}