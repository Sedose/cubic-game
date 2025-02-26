use macroquad::models::Vertex;

use super::*;

#[path = "mesh.rs"]
mod mesh;
use mesh::*;

#[rustfmt::skip]
pub fn build_chunk_meshes(
    chunks: impl IntoIterator<Item = (ChunkPos, ChunkModel)>,
    atlas: Option<Texture2D>,
) -> impl Iterator<Item = Mesh> {
    let mut meshes = Meshes::new(atlas);

    for (chunk_pos, chunk_model) in chunks {
        if chunk_model.is_empty() {
            return meshes.into_iter();
        }

        let world_pos: BlockPos = chunk_pos.into();

        for y in 0..CHUNK_SIZE_16 {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let block_model: &BlockModel = chunk_model.get(x, y, z);
                    let block_pos = BlockPos {
                        x: x as isize + world_pos.x,
                        y: y as isize + world_pos.y,
                        z: z as isize + world_pos.z,
                    };
                    process_block_model(block_model, block_pos, &mut meshes);
                }
            }
        }
    }
    meshes.into_iter()
}

fn process_block_model(block_model: &BlockModel, block_pos: BlockPos, meshes: &mut Meshes) {
    use BlockModel::*;

    let mut add_faces = |texture, faces: &[fn(BlockPos, UvTexture) -> [Vertex; 4]]| {
        meshes.extend_with(block_pos, texture, faces);
    };

    match *block_model {
        Empty | NonCube => {}
        Top(texture) => add_faces(texture, &[top_vert]),
        Bottom(texture) => add_faces(texture, &[bottom_vert]),
        Px(texture) => add_faces(texture, &[px_vert]),
        Nx(texture) => add_faces(texture, &[nx_vert]),
        Pz(texture) => add_faces(texture, &[pz_vert]),
        Nz(texture) => add_faces(texture, &[nz_vert]),
        TopPx(texture) => add_faces(texture, &[top_vert, px_vert]),
        TopNx(texture) => add_faces(texture, &[top_vert, nx_vert]),
        TopPz(texture) => add_faces(texture, &[top_vert, pz_vert]),
        TopNz(texture) => add_faces(texture, &[top_vert, nz_vert]),
        BottomPx(texture) => add_faces(texture, &[bottom_vert, px_vert]),
        BottomNx(texture) => add_faces(texture, &[bottom_vert, nx_vert]),
        BottomPz(texture) => add_faces(texture, &[bottom_vert, pz_vert]),
        BottomNz(texture) => add_faces(texture, &[bottom_vert, nz_vert]),
        PxPz(texture) => add_faces(texture, &[px_vert, pz_vert]),
        PxNz(texture) => add_faces(texture, &[px_vert, nz_vert]),
        NxPz(texture) => add_faces(texture, &[nx_vert, pz_vert]),
        NxNz(texture) => add_faces(texture, &[nx_vert, nz_vert]),
        _ => {}, // Other cases (e.g., double-sided textures) can be handled here later.
    }
}

#[rustfmt::skip]
const fn top_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_right()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.low_right()),
    ]
}

#[rustfmt::skip]
const fn bottom_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.up_right()),
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_right()),
    ]
}

#[rustfmt::skip]
const fn px_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.low_right()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_right()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.up_left()),
    ]
}

#[rustfmt::skip]
const fn nx_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_left()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.up_right()),
    ]
}

#[rustfmt::skip]
const fn pz_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.up_right()),
    ]
}

#[rustfmt::skip]
const fn nz_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.up_right()),
    ]
}
