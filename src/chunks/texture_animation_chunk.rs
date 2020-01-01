use crate::chunks::{BytesTotalSize, TextureRotation, TextureScaling, TextureTranslation};
use crate::consts::{KTAR_TAG, KTAS_TAG, KTAT_TAG};
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct TextureAnimationChunk {
    pub chunk_size: u32,

    pub data: Vec<TextureAnimation>,
}

calculate_chunk_size_impl!(TextureAnimationChunk);

impl ctx::TryFromCtx<'_, Endian> for TextureAnimationChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let texture_animation = src.gread_with::<TextureAnimation>(offset, ctx)?;
            total_size += texture_animation.inclusive_size;
            data.push(texture_animation);
        }

        Ok((TextureAnimationChunk { chunk_size, data }, *offset))
    }
}

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

impl BytesTotalSize for TextureAnimationChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for texture_animation in &self.data {
            result += texture_animation.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct TextureAnimation {
    pub inclusive_size: u32,

    pub texture_translation: Option<TextureTranslation>,
    pub texture_rotation: Option<TextureRotation>,
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
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);

            match tag {
                KTAT_TAG => {
                    let ktat = src.gread_with::<TextureTranslation>(offset, ctx)?;
                    //dbg!(&ktat);
                    texture_animation.texture_translation = Some(ktat);
                }
                KTAR_TAG => {
                    let ktar = src.gread_with::<TextureRotation>(offset, ctx)?;
                    //dbg!(&ktar);
                    texture_animation.texture_rotation = Some(ktar);
                }
                KTAS_TAG => {
                    let ktas = src.gread_with::<TextureScaling>(offset, ctx)?;
                    //dbg!(&ktas);
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

impl BytesTotalSize for TextureAnimation {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        if self.texture_translation.is_some() {
            result += 4;
            result += self
                .texture_translation
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.texture_rotation.is_some() {
            result += 4;
            result += self.texture_rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.texture_scaling.is_some() {
            result += 4;
            result += self.texture_scaling.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
