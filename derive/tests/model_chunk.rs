use nebula_mdx::consts::MODL_TAG;
use nebula_mdx_internal::NMread;
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};

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
    buffer
        .pwrite_with::<u64>(8386058079685669444u64, 8, LE)
        .unwrap();
    buffer.pwrite_with::<u32>(800, 344, LE).unwrap();

    let chunk: ModelChunk = buffer.pread_with(0, LE).unwrap();
    dbg!("{:?}", &chunk);
}
