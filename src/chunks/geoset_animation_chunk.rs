use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::{BytesTotalSize, GeosetAlpha, GeosetColor};
use std::mem::size_of_val;
use crate::consts::{KGAO_TAG, KGAC_TAG};

#[derive(PartialEq, Debug)]
pub struct GeosetAnimationChunk {
    pub chunk_size: u32,

    pub data: Vec<GeosetAnimation>,
}

calculate_chunk_size_impl!(GeosetAnimationChunk);

impl ctx::TryFromCtx<'_, Endian> for GeosetAnimationChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let geoset_animation = src.gread_with::<GeosetAnimation>(offset, ctx)?;
            total_size += geoset_animation.inclusive_size;
            data.push(geoset_animation);
        }

        Ok((GeosetAnimationChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for GeosetAnimationChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for geoset_animation in self.data {
            src.gwrite_with::<GeosetAnimation>(geoset_animation, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetAnimationChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for geoset_animation in &self.data {
            result += geoset_animation.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct GeosetAnimation {
    pub inclusive_size: u32,

    pub alpha: f32,
    pub flags: u32,
    pub color: [f32; 3], // bgr
    pub geoset_id: u32,

    pub geoset_alpha: Option<GeosetAlpha>,
    pub geoset_color: Option<GeosetColor>,
}

impl ctx::TryFromCtx<'_, Endian> for GeosetAnimation {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;
        let alpha = src.gread_with::<f32>(offset, ctx)?;
        let flags = src.gread_with::<u32>(offset, ctx)?;
        let color = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let geoset_id = src.gread_with::<u32>(offset, ctx)?;

        let mut geoset_animation = GeosetAnimation {
            inclusive_size,
            alpha,
            flags,
            color,
            geoset_id,
            geoset_alpha: None,
            geoset_color: None
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();
            dbg!(format!("{:X}", &tag));
            dbg!(&tag);

            match tag {
                KGAO_TAG => {
                    let geoset_alpha = src.gread_with::<GeosetAlpha>(offset, ctx)?;
                    geoset_animation.geoset_alpha = Some(geoset_alpha);
                },
                KGAC_TAG => {
                    let geoset_color = src.gread_with::<GeosetColor>(offset, ctx)?;
                    geoset_animation.geoset_color = Some(geoset_color);
                },
                _ => unreachable!(),
            }
        }

        Ok((geoset_animation, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for GeosetAnimation {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;
        src.gwrite_with::<f32>(self.alpha, offset, ctx)?;
        src.gwrite_with::<u32>(self.flags, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.color[id], offset, ctx)?;
        }
        src.gwrite_with::<u32>(self.geoset_id, offset, ctx)?;

        if self.geoset_alpha.is_some() {
            src.gwrite_with::<u32>(KGAO_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetAlpha>(self.geoset_alpha.unwrap(), offset, ctx)?;
        }
        if self.geoset_color.is_some() {
            src.gwrite_with::<u32>(KGAC_TAG, offset, ctx)?;
            src.gwrite_with::<GeosetColor>(self.geoset_color.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetAnimation {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);
        result += size_of_val(&self.alpha);
        result += size_of_val(&self.flags);
        result += size_of_val(&self.color);
        result += size_of_val(&self.geoset_id);

        if self.geoset_alpha.is_some() {
            result += 4;
            result += self.geoset_alpha.as_ref().unwrap().total_bytes_size();
        }
        if self.geoset_color.is_some() {
            result += 4;
            result += self.geoset_color.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}