use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::BytesTotalSize;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct PivotPointChunk {
    pub chunk_size: u32,

    // chunk_size / 12
    pub data: Vec<PivotPoint>,
}

calculate_chunk_size_impl!(PivotPointChunk);

impl ctx::TryFromCtx<'_, Endian> for PivotPointChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(pivot_point_count) = u32::checked_div(chunk_size.clone(), 12) {
            for _ in 0..pivot_point_count {
                let pivot_point = src.gread_with::<PivotPoint>(offset, ctx)?;
                data.push(pivot_point);
            }
        }

        Ok((PivotPointChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for PivotPointChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for pivot_point in self.data {
            src.gwrite_with::<PivotPoint>(pivot_point, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for PivotPointChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for pivot_point in &self.data {
            result += pivot_point.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct PivotPoint {
    pub position: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for PivotPoint {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let position = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((PivotPoint {
            position,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for PivotPoint {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        for id in 0..3 {
            src.gwrite_with::<f32>(self.position[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for PivotPoint {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.position);

        result
    }
}