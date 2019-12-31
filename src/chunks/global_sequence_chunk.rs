use scroll::{ctx, Endian, Pread, Pwrite};
use crate::chunks::BytesTotalSize;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct GlobalSequenceChunk {
    pub chunk_size: u32,

    // chunk_size / 4
    pub data: Vec<GlobalSequence>,
}

calculate_chunk_size_impl!(GlobalSequenceChunk);

impl ctx::TryFromCtx<'_, Endian> for GlobalSequenceChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;
        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        if let Some(sequence_count) = u32::checked_div(chunk_size.clone(), 4) {
            for _ in 0..sequence_count {
                let sequence = src.gread_with::<GlobalSequence>(offset, ctx)?;
                data.push(sequence);
            }
        }

        Ok((GlobalSequenceChunk {
            chunk_size,
            data,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for GlobalSequenceChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for global_sequence in self.data {
            src.gwrite_with::<GlobalSequence>(global_sequence, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GlobalSequenceChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for global_sequence in &self.data {
            result += global_sequence.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct GlobalSequence {
    pub duration: u32,
}

impl ctx::TryFromCtx<'_, Endian> for GlobalSequence {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let duration = src.gread_with::<u32>(offset, ctx)?;

        Ok((GlobalSequence {
            duration,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for GlobalSequence {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.duration, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for GlobalSequence {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.duration);

        result
    }
}