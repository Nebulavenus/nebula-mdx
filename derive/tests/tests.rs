use nebula_mdx_derive_internal::NMread;

#[derive(NMread)]
pub struct VersionChunk {
    pub chunk_size: u32,
    pub version: u32,
}

#[test]
fn version_chunk_read_test() {
    let chunk = VersionChunk { chunk_size: 0, version: 0 };
}

