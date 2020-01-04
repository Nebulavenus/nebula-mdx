use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct Extent {
    pub bounds_radius: f32,
    pub minimum_extent: [f32; 3],
    pub maximum_extent: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for Extent {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let bounds_radius = src.gread_with::<f32>(offset, ctx)?;
        let minimum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];
        let maximum_extent = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((Extent {
            bounds_radius,
            minimum_extent,
            maximum_extent,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Extent {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<f32>(self.bounds_radius, offset, ctx)?;
        for id in 0..3 {
            src.gwrite_with::<f32>(self.minimum_extent[id], offset, ctx)?;
        }
        for id in 0..3 {
            src.gwrite_with::<f32>(self.maximum_extent[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Extent {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.bounds_radius);
        result += size_of_val(&self.minimum_extent);
        result += size_of_val(&self.maximum_extent);

        result
    }
}