use nebula_mdx_internal::{NMread, NMbts};
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};

pub trait BytesTotalSize {
    fn total_bytes_size(&self) -> usize;
}

const GEOS_TAG: u32 = 1397704007;

#[derive(NMread, NMbts, Debug)]
pub struct GeosChunk {
    #[nebula(tag = "GEOS_TAG")]
    pub chunk_size: u32,

    #[nebula(behaviour = "inclusive")]
    pub data: Vec<Geoset>,
}

const VRTX_TAG: u32 = 1481921110;

#[derive(NMread, NMbts, Debug)]
pub struct Geoset {
    pub inclusive_size: u32,

    #[nebula(tag = "VRTX_TAG")]
    //pub vertex_count: u32, // this is now inside vertex_positions
    #[nebula(behaviour = "normal")]
    pub vertex_positions: Vec<VertexPosition>,
}

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct VertexPosition {
    pub position: [f32; 3],
}

#[test]
fn geos_chunk_read_test() {
    use std::include_bytes;

    let bytes = include_bytes!("../../testfiles/geos_chunk.mdx");

    let chunk: GeosChunk = bytes.pread_with(0, LE).unwrap();

    assert_eq!(bytes.len(), chunk.total_bytes_size());
}