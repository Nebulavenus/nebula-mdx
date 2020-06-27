use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug, Clone)]
pub struct Track<T> {
    pub time: u32,
    pub value: T,
    pub in_tan: Option<T>,
    pub out_tan: Option<T>,
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, Endian> for Track<T>
where
    T: ctx::TryFromCtx<'a, Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let time = src.gread_with::<u32>(offset, ctx)?;
        let value = src.gread_with::<T>(offset, ctx)?;

        Ok((
            Track {
                time,
                value,
                in_tan: None,
                out_tan: None,
            },
            *offset,
        ))
    }
}

impl<T> ctx::TryIntoCtx<Endian> for Track<T>
where
    T: ctx::TryIntoCtx<Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.time, offset, ctx)?;
        src.gwrite_with::<T>(self.value, offset, ctx)?;

        if self.in_tan.is_some() {
            src.gwrite_with::<T>(self.in_tan.unwrap(), offset, ctx)?;
        }

        if self.out_tan.is_some() {
            src.gwrite_with::<T>(self.out_tan.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl<T> BytesTotalSize for Track<T> {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;
        result += size_of_val(&self.time);
        result += size_of_val(&self.value);
        if self.in_tan.is_some() {
            result += size_of_val(self.in_tan.as_ref().unwrap());
        }
        if self.out_tan.is_some() {
            result += size_of_val(self.out_tan.as_ref().unwrap());
        }
        result
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Transform<T> {
    pub number_of_tracks: u32,
    pub interpolation_type: u32,
    pub global_sequence_id: u32,

    pub data: Vec<Track<T>>,
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, Endian> for Transform<T>
where
    T: ctx::TryFromCtx<'a, Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_from_ctx(src: &'a [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let number_of_tracks = src.gread_with::<u32>(offset, ctx)?;
        let interpolation_type = src.gread_with::<u32>(offset, ctx)?;
        let global_sequence_id = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        for _ in 0..number_of_tracks {
            let mut track = src.gread_with::<Track<T>>(offset, ctx)?;

            if interpolation_type > 1 {
                let in_tan = src.gread_with::<T>(offset, ctx)?;
                let out_tan = src.gread_with::<T>(offset, ctx)?;

                track.in_tan = Some(in_tan);
                track.out_tan = Some(out_tan);
            }
            data.push(track);
        }

        Ok((
            Transform {
                number_of_tracks,
                interpolation_type,
                global_sequence_id,
                data,
            },
            *offset,
        ))
    }
}

impl<T> ctx::TryIntoCtx<Endian> for Transform<T>
where
    T: ctx::TryIntoCtx<Endian, Error = scroll::Error>,
{
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.number_of_tracks, offset, ctx)?;
        src.gwrite_with::<u32>(self.interpolation_type, offset, ctx)?;
        src.gwrite_with::<u32>(self.global_sequence_id, offset, ctx)?;

        for track in self.data {
            src.gwrite_with::<Track<T>>(track, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl<T> BytesTotalSize for Transform<T> {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.number_of_tracks);
        result += size_of_val(&self.interpolation_type);
        result += size_of_val(&self.global_sequence_id);

        for track in &self.data {
            result += track.total_bytes_size();
        }

        result
    }
}
