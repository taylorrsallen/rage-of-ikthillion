use crate::*;

//================================-================================-================================
pub const TEXTURE_ATLAS_DIM: usize = 32;
pub const TEXTURE_ATLAS_UV_DIM: f32 = 1.0 / TEXTURE_ATLAS_DIM as f32;

pub const CUBE_VERTICES: [[f32; 3]; 8] = [
    [-0.5, -0.5, -0.5], // 0
    [ 0.5, -0.5, -0.5], // 1
    [-0.5,  0.5, -0.5], // 2
    [ 0.5,  0.5, -0.5], // 3
    [-0.5, -0.5,  0.5], // 4
    [ 0.5, -0.5,  0.5], // 5
    [-0.5,  0.5,  0.5], // 6
    [ 0.5,  0.5,  0.5], // 7
];
pub const CUBE_NORMALS: [[f32; 3]; 6] = [
    [-1.0,  0.0,  0.0], // Left face
    [ 1.0,  0.0,  0.0], // Right face
    [ 0.0, -1.0,  0.0], // Bottom face
    [ 0.0,  1.0,  0.0], // Top face
    [ 0.0,  0.0, -1.0], // Back face
    [ 0.0,  0.0,  1.0], // Front face
];
pub const CUBE_UVS: [[f32; 2]; 4] = [
    [TEXTURE_ATLAS_UV_DIM - 0.0001, TEXTURE_ATLAS_UV_DIM - 0.0001],
    [0.0001                       , TEXTURE_ATLAS_UV_DIM - 0.0001],
    [TEXTURE_ATLAS_UV_DIM - 0.0001, 0.0001                 ],
    [0.0001                       , 0.0001                 ],
    ];
pub const CUBE_FACE_VERTICES: [[usize; 4]; 6] = [ // Read as: 0, 1, 2, 2, 1, 3
    [4, 0, 6, 2], // Left face
    [1, 5, 3, 7], // Right face
    [4, 5, 0, 1], // Bottom face
    [2, 3, 6, 7], // Top face
    [0, 1, 2, 3], // Back face
    [5, 4, 7, 6], // Front face
    ];
    pub const CUBE_FACE_INDICES: [u32; 6] = [0, 2, 1, 1, 2, 3];
    pub const FACE_CHECKS: [IVec3; 6] = [
        IVec3::new(-1,  0,  0), // Left face
        IVec3::new( 1,  0,  0), // Right face
        IVec3::new( 0, -1,  0), // Bottom face
        IVec3::new( 0,  1,  0), // Top face
        IVec3::new( 0,  0, -1), // Back face
        IVec3::new( 0,  0,  1), // Front face
    ];

//================================-================================-================================
pub const MATTER_TEXTURE_IDS: [[u32; 6]; 3] = [
    [6, 6, 7, 8, 6, 6],
    [0, 0, 1, 2, 0, 0],
    [3, 3, 4, 5, 3, 3],
];

//================================-================================-================================
#[derive(Default)]
pub struct MeshData {
    vertices: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    vertex_indices: Vec<u32>,
    vertex_count: u32,
}

impl MeshData {
    pub fn get_mesh(
        mut self,
    ) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.set_indices(Some(Indices::U32(self.vertex_indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        mesh
    }

    pub fn cut_cube_from_mesh(
        &mut self,
        matter: TileMatter,
        local_coord: &IVec3,
        global_coord: &IVec3,
        grid_root: &GridRoot,
    ) {
        let neighbors = grid_root.get_tile_neighbors(global_coord);
    
        for face in 0..6 {
            if neighbors[face].is_solid() {
                self.add_cube_to_mesh(neighbors[face].matter, &(*local_coord + FACE_CHECKS[face]), &(*global_coord + FACE_CHECKS[face]), grid_root);
            }
        }
    }

    pub fn add_cube_to_mesh(
        &mut self,
        matter: TileMatter,
        local_coord: &IVec3,
        global_coord: &IVec3,
        grid_root: &GridRoot,
    ) {
        let neighbors = grid_root.get_tile_neighbors(global_coord);
    
        for face in 0..6 {
            if neighbors[face].is_solid() {
                continue;
            }
    
            for vertex_index in 0..4 {
                let vertex = CUBE_VERTICES[CUBE_FACE_VERTICES[face][vertex_index]];
                self.vertices.push([
                    vertex[0] + local_coord.x as f32,
                    vertex[1] + local_coord.y as f32,
                    vertex[2] + local_coord.z as f32,
                ]);
    
                let uv = CUBE_UVS[vertex_index];
                let uv_offset = match global_coord.y {
                        -2 => { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[1].block_texture_ids[face] as f32 }, // Dark
                        -3 => { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[2].block_texture_ids[face] as f32 }, // Darker
                        -4 => { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[3].block_texture_ids[face] as f32 }, // Darkest
                        -5 => { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[4].block_texture_ids[face] as f32 }, // Voidstone
                        -6 => { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[5].block_texture_ids[face] as f32 }, // Void
                        _ =>  { TEXTURE_ATLAS_UV_DIM * TILE_MATTER_DEFS[matter as usize].block_texture_ids[face] as f32 },
                    };
                
                
                let uv_offset_floor = uv_offset.floor() as f32;
                self.uvs.push([
                    uv[0] + uv_offset - uv_offset_floor,
                    uv[1] + uv_offset_floor * TEXTURE_ATLAS_UV_DIM as f32,
                ]);
            }
    
            self.normals.extend(vec![CUBE_NORMALS[face]; 4]);
    
            for tri_index in 0..6 {
                self.vertex_indices.push(CUBE_FACE_INDICES[tri_index] + self.vertex_count);
            }
    
            self.vertex_count += 4;
        }
    }
}