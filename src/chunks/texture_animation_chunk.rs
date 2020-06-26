use crate::chunks::{BytesTotalSize, Transform, Vec3, Vec4};
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

    pub translation: Option<Transform<Vec3>>,
    pub rotation: Option<Transform<Vec4>>,
    pub scaling: Option<Transform<Vec3>>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureAnimation {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx).unwrap();
        let mut texture_animation = TextureAnimation {
            inclusive_size,
            translation: None,
            rotation: None,
            scaling: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KTAT_TAG => {
                    let ktat = src.gread_with(offset, ctx)?;
                    texture_animation.translation = Some(ktat);
                }
                KTAR_TAG => {
                    let ktar = src.gread_with(offset, ctx)?;
                    texture_animation.rotation = Some(ktar);
                }
                KTAS_TAG => {
                    let ktas = src.gread_with(offset, ctx)?;
                    texture_animation.scaling = Some(ktas);
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

        if self.translation.is_some() {
            src.gwrite_with::<u32>(KTAT_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec3>>(self.translation.unwrap(), offset, ctx)?;
        }
        if self.rotation.is_some() {
            src.gwrite_with::<u32>(KTAR_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec4>>(self.rotation.unwrap(), offset, ctx)?;
        }
        if self.scaling.is_some() {
            src.gwrite_with::<u32>(KTAS_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Vec3>>(self.scaling.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureAnimation {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        if self.translation.is_some() {
            result += 4;
            result += self.translation.as_ref().unwrap().total_bytes_size();
        }
        if self.rotation.is_some() {
            result += 4;
            result += self.rotation.as_ref().unwrap().total_bytes_size();
        }
        if self.scaling.is_some() {
            result += 4;
            result += self.scaling.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
