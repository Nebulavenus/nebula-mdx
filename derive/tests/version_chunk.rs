use nebula_mdx_internal::NMread;
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};
use nebula_mdx::consts::VERS_TAG;

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