use nebula_mdx_derive_internal::NMread;
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};

const GEOS_TAG: u32 = 1397704007;

#[derive(NMread, Debug)]
pub struct GeosChunk {
    #[nebula(tag = "GEOS_TAG")]
    pub chunk_size: u32,

    #[nebula(behaviour = "inclusive")]
    pub data: Vec<Geoset>,
}

const VRTX_TAG: u32 = 1481921110;

#[derive(NMread, Debug)]
pub struct Geoset {
    pub inclusize_size: u32,

    #[nebula(tag = "VRTX_TAG")]
    pub vertex_count: u32,
    #[nebula(behaviour = "normal")]
    pub vertex_positions: Vec<VertexPosition>,
}

#[derive(NMread, PartialEq, Debug)]
pub struct VertexPosition {
    pub position: [f32; 3],
}

#[test]
fn geos_chunk_read_test() {
    let mut buffer = [0u8; 348];
    buffer.pwrite_with::<u32>(1397704007u32, 0, LE).unwrap();
    buffer.pwrite_with::<u32>(27079, 4, LE).unwrap();
    buffer.pwrite_with::<u32>(22596, 8, LE).unwrap();
    buffer.pwrite_with::<u32>(800, 344, LE).unwrap();

    let chunk: GeosChunk = buffer.pread_with(0, LE).unwrap();
    dbg!("{:?}", &chunk);
}