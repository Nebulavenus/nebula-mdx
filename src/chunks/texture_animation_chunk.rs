use crate::chunks::{BytesTotalSize, TextureRotation, TextureScaling, TextureTranslation};
use crate::consts::{KTAR_TAG, KTAS_TAG, KTAT_TAG};
use scroll::{ctx, Endian, Pread, Pwrite};
use nebula_mdx_internal::{NMread, NMbts};

#[derive(NMread, NMbts, PartialEq, Debug)]
pub struct TextureAnimationChunk {
    pub chunk_size: u32,

    #[nebula(behaviour = "inclusive")]
    pub data: Vec<TextureAnimation>,
}

calculate_chunk_size_impl!(TextureAnimationChunk);

impl ctx::TryIntoCtx<Endian> for TextureAnimationChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for texture_animation in self.data {
            src.gwrite_with::<TextureAnimation>(texture_animation, offset, ctx)?;
        }

        Ok(*offset)
    }
}

#[derive(NMbts, PartialEq, Debug)]
pub struct TextureAnimation {
    pub inclusive_size: u32,

    #[nebula(tag = "KTAT_TAG")]
    #[nebula(order = "unknown_tag")]
    pub texture_translation: Option<TextureTranslation>,
    #[nebula(tag = "KTAR_TAG")]
    #[nebula(order = "unknown_tag")]
    pub texture_rotation: Option<TextureRotation>,
    #[nebula(tag = "KTAS_TAG")]
    #[nebula(order = "unknown_tag")]
    pub texture_scaling: Option<TextureScaling>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureAnimation {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx).unwrap();
        let mut texture_animation = TextureAnimation {
            inclusive_size,
            texture_translation: None,
            texture_rotation: None,
            texture_scaling: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KTAT_TAG => {
                    let ktat = src.gread_with::<TextureTranslation>(offset, ctx)?;
                    texture_animation.texture_translation = Some(ktat);
                }
                KTAR_TAG => {
                    let ktar = src.gread_with::<TextureRotation>(offset, ctx)?;
                    texture_animation.texture_rotation = Some(ktar);
                }
                KTAS_TAG => {
                    let ktas = src.gread_with::<TextureScaling>(offset, ctx)?;
                    texture_animation.texture_scaling = Some(ktas);
                }
                _ => unreachable!(),
            }
        }

        Ok((texture_animation, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for TextureAnimation {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        if self.texture_translation.is_some() {
            src.gwrite_with::<u32>(KTAT_TAG, offset, ctx)?;
            src.gwrite_with::<TextureTranslation>(self.texture_translation.unwrap(), offset, ctx)?;
        }
        if self.texture_rotation.is_some() {
            src.gwrite_with::<u32>(KTAR_TAG, offset, ctx)?;
            src.gwrite_with::<TextureRotation>(self.texture_rotation.unwrap(), offset, ctx)?;
        }
        if self.texture_scaling.is_some() {
            src.gwrite_with::<u32>(KTAS_TAG, offset, ctx)?;
            src.gwrite_with::<TextureScaling>(self.texture_scaling.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}