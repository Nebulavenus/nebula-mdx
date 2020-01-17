use nebula_mdx_internal::{NMread, NMbts};
#[allow(unused_imports)]
use scroll::{Pread, Pwrite, LE};
use nebula_mdx::chunks::{BytesTotalSize, TextureRotation, TextureScaling, TextureTranslation};
use nebula_mdx::consts::{TXAN_TAG, KTAR_TAG, KTAS_TAG, KTAT_TAG};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct TextureAnimationChunk {
    #[nebula(tag = "TXAN_TAG")]
    pub chunk_size: u32,

    #[nebula(behaviour = "inclusive")]
    pub data: Vec<TextureAnimation>,
}

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct TextureAnimation {
    pub inclusive_size: u32,

    //#[nebula(tag = "KTAT_TAG")]
    //pub texture_translation: Option<TextureTranslation>,
    //#[nebula(tag = "KTAR_TAG")]
    //pub texture_rotation: Option<TextureRotation>,
    //#[nebula(tag = "KTAS_TAG")]
    //pub texture_scaling: Option<TextureScaling>,
}

#[test]
fn texture_animation_chunk_read_test() {

    let bytes = include_bytes!("../../testfiles/txan_chunk.mdx");

    let chunk: TextureAnimationChunk = bytes.pread_with(0, LE).unwrap();
    dbg!("{:?}", &chunk);
    dbg!(bytes.len());

    assert_eq!(bytes.len(), chunk.total_bytes_size());
}