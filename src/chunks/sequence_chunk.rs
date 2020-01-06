use crate::chunks::BytesTotalSize;
use crate::chunks::Extent;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct SequenceChunk {
    pub chunk_size: u32,

    // chunk_size / 132
    pub data: Vec<Sequence>,
}

calculate_chunk_size_impl!(SequenceChunk);

impl ctx::TryFromCtx<'_, Endian> for SequenceChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(sequence_count) = u32::checked_div(chunk_size.clone(), 132) {
            for _ in 0..sequence_count {
                let sequence = src.gread_with::<Sequence>(offset, ctx)?;
                data.push(sequence);
            }
        }

        Ok((SequenceChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for SequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for sequence in self.data {
            src.gwrite_with::<Sequence>(sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for SequenceChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for sequence in &self.data {
            result += sequence.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Sequence {
    pub name: String,
    pub interval_start: u32,
    pub interval_end: u32,
    pub move_speed: f32,
    pub non_looping: u32,
    pub rarity: f32,
    pub unknown: u32,
    pub extent: Extent,
}

impl ctx::TryFromCtx<'_, Endian> for Sequence {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        // Name has fixed size
        let max_name_len = 80usize;
        let name = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let interval_start = src.gread_with::<u32>(offset, ctx)?;
        let interval_end = src.gread_with::<u32>(offset, ctx)?;
        let move_speed = src.gread_with::<f32>(offset, ctx)?;
        let non_looping = src.gread_with::<u32>(offset, ctx)?;
        let rarity = src.gread_with::<f32>(offset, ctx)?;
        let unknown = src.gread_with::<u32>(offset, ctx)?;
        let extent = src.gread_with::<Extent>(offset, ctx)?;

        Ok((
            Sequence {
                name,
                interval_start,
                interval_end,
                move_speed,
                non_looping,
                rarity,
                unknown,
                extent,
            },
            *offset,
        ))
    }
}

impl ctx::TryIntoCtx<Endian> for Sequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        // Name has fixed size
        let max_name_len = 80usize;
        let null_offset = &mut 0usize;
        for _ in 0..max_name_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.name.as_ref(), &mut offset.clone(), ())?
            .to_string();
        *offset += max_name_len;

        src.gwrite_with::<u32>(self.interval_start, offset, ctx)?;
        src.gwrite_with::<u32>(self.interval_end, offset, ctx)?;
        src.gwrite_with::<f32>(self.move_speed, offset, ctx)?;
        src.gwrite_with::<u32>(self.non_looping, offset, ctx)?;
        src.gwrite_with::<f32>(self.rarity, offset, ctx)?;
        src.gwrite_with::<u32>(self.unknown, offset, ctx)?;
        src.gwrite_with::<Extent>(self.extent, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Sequence {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        let max_name_len = 80usize;
        result += max_name_len;

        result += size_of_val(&self.interval_start);
        result += size_of_val(&self.interval_end);
        result += size_of_val(&self.move_speed);
        result += size_of_val(&self.non_looping);
        result += size_of_val(&self.rarity);
        result += size_of_val(&self.unknown);
        result += &self.extent.total_bytes_size();

        result
    }
}
