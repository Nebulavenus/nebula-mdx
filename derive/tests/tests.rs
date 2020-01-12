use nebula_mdx_derive_internal::NMread;
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};

#[allow(dead_code)]
const VERS_TAG: u32 = 432412312;

#[derive(NMread, Debug)]
pub struct VersionChunk {
    #[nebula(tag = VERS_TAG)]
    pub chunk_size: u32,
    pub version: u32,
}

#[test]
fn version_chunk_read_test() {
    let bytes = [0xefu8, 0xbe, 0xad, 0xde, 142, 3, 0, 0];
    let chunk: VersionChunk = bytes.pread_with(0, LE).unwrap();
    println!("{:?}", &chunk);
}

