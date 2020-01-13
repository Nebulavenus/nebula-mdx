use nebula_mdx_derive_internal::NMread;
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};

const VERS_TAG: u32 = 1397900630;

#[derive(NMread, Debug)]
pub struct VersionChunk {
    #[nebula(tag = "VERS_TAG")]
    pub chunk_size: u32,
    pub version: u32,
}

#[test]
fn version_chunk_read_test() {
    let mut buffer = [0u8; 12];
    buffer.pwrite_with::<u32>(1397900630u32, 0, LE).unwrap();
    buffer.pwrite_with::<u32>(1235121351u32, 4, LE).unwrap();
    buffer.pwrite_with::<u32>(800, 8, LE).unwrap();

    let chunk: VersionChunk = buffer.pread_with(0, LE).unwrap();
    dbg!("{:?}", &chunk);
}

const MODL_TAG: u32 = 1279545165;

#[derive(NMread, Debug)]
pub struct ModelChunk {
    #[nebula(tag = "MODL_TAG")]
    pub chunk_size: u32,

    #[nebula(length = "336")]
    pub name: String,
    pub unknown: u32,
}

#[test]
fn model_chunk_read_test() {
    let mut buffer = [0u8; 348];
    buffer.pwrite_with::<u32>(1279545165u32, 0, LE).unwrap();
    buffer.pwrite_with::<u32>(372u32, 4, LE).unwrap();
    buffer.pwrite_with::<u64>(8386058079685669444u64, 8, LE).unwrap();
    buffer.pwrite_with::<u32>(800, 344, LE).unwrap();

    let chunk: ModelChunk = buffer.pread_with(0, LE).unwrap();
    dbg!("{:?}", &chunk);
}