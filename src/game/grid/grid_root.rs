use crate::*;

//================================-================================-================================
#[derive(Default, Component, Clone)]
pub struct GridRoot {
    pub chunks: HashMap<IVec3, GridChunk>,

    background: GridTile,
}

impl GridRoot {
    pub fn new(
        background: GridTile,
    ) -> Self {
        Self {
            background,
            ..default()
        }
    }

    pub fn key_from_coord(
        coord: &IVec3,
    ) -> IVec3 {
        *coord & CHUNK_ORIGIN_MASK
    }

    pub fn get_chunk_from_key(
        &mut self,
        key: &IVec3,
    ) -> &GridChunk {
        if self.chunks.contains_key(&key) {
            self.chunks.get(&key).unwrap()
        } else {
            self.chunks.insert(*key, GridChunk::new(&key, self.background));
            self.chunks.get(&key).unwrap()
        }
    }

    pub fn get_mut_chunk_from_key(
        &mut self,
        key: &IVec3,
    ) -> &mut GridChunk {
        if self.chunks.contains_key(&key) {
            self.chunks.get_mut(&key).unwrap()
        } else {
            self.chunks.insert(*key, GridChunk::new(&key, self.background));
            self.chunks.get_mut(&key).unwrap()
        }
    }



    pub fn get_tile_from_coord(
        &self,
        coord: &IVec3,
    ) -> &GridTile {
        let key = GridRoot::key_from_coord(coord);
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.get_tile_from_coord(coord)
        } else {
            &self.background
        }
    }

    pub fn get_mut_tile_from_coord(
        &mut self,
        coord: &IVec3,
    ) -> &mut GridTile {
        self.get_mut_chunk_from_key(&GridRoot::key_from_coord(coord)).get_mut_tile_from_coord(coord)
    }

    pub fn set_tile_on_at_coord(
        &mut self,
        coord: &IVec3,
        tile: &GridTile,
    ) {
        self.get_mut_chunk_from_key(&GridRoot::key_from_coord(coord)).set_tile_on_at_coord(coord, tile);
    }

    pub fn set_tile_off_at_coord(
        &mut self,
        coord: &IVec3,
    ) {
        self.get_mut_chunk_from_key(&GridRoot::key_from_coord(coord)).set_tile_off_at_coord(coord);
    }



    pub fn get_creature_from_coord(
        &self,
        coord: &IVec3,
    ) -> &Option<Entity> {
        let key = GridRoot::key_from_coord(coord);
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.get_creature_from_coord(coord)
        } else {
            &None
        }
    }

    pub fn set_creature_at_coord(
        &mut self,
        coord: &IVec3,
        entity: &Option<Entity>,
    ) {
        self.get_mut_chunk_from_key(&GridRoot::key_from_coord(coord)).set_creature_at_coord(coord, entity);
    }



    pub fn get_item_from_coord(
        &self,
        coord: &IVec3,
    ) -> &Option<Entity> {
        let key = GridRoot::key_from_coord(coord);
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.get_item_from_coord(coord)
        } else {
            &None
        }
    }

    pub fn set_item_at_coord(
        &mut self,
        coord: &IVec3,
        entity: &Option<Entity>,
    ) {
        self.get_mut_chunk_from_key(&GridRoot::key_from_coord(coord)).set_item_at_coord(coord, entity);
    }



    pub fn despawn_chunk_scenes(
        &mut self,
        commands: &mut Commands,
    ) {
        for (_, chunk) in self.chunks.iter_mut() {
            chunk.despawn_tile_scenes(commands);
        }
    }

    pub fn get_tile_neighbors(
        &self,
        tile_coord: &IVec3,
    ) -> [&GridTile; 6] {
        [
            self.get_tile_from_coord(&IVec3::new(tile_coord.x - 1, tile_coord.y, tile_coord.z)),
            self.get_tile_from_coord(&IVec3::new(tile_coord.x + 1, tile_coord.y, tile_coord.z)),
            self.get_tile_from_coord(&IVec3::new(tile_coord.x, tile_coord.y - 1, tile_coord.z)),
            self.get_tile_from_coord(&IVec3::new(tile_coord.x, tile_coord.y + 1, tile_coord.z)),
            self.get_tile_from_coord(&IVec3::new(tile_coord.x, tile_coord.y, tile_coord.z - 1)),
            self.get_tile_from_coord(&IVec3::new(tile_coord.x, tile_coord.y, tile_coord.z + 1)),
        ]
    }
}