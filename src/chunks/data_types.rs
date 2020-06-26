use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl ctx::TryFromCtx<'_, Endian> for Vec2 {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let x = src.gread_with(offset, ctx)?;
        let y = src.gread_with(offset, ctx)?;

        Ok((Vec2 { x, y }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Vec2 {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<f32>(self.x, offset, ctx)?;
        src.gwrite_with::<f32>(self.y, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Vec2 {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;
        result += size_of_val(&self.x);
        result += size_of_val(&self.y);
        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ctx::TryFromCtx<'_, Endian> for Vec3 {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let x = src.gread_with(offset, ctx)?;
        let y = src.gread_with(offset, ctx)?;
        let z = src.gread_with(offset, ctx)?;

        Ok((Vec3 { x, y, z }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Vec3 {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<f32>(self.x, offset, ctx)?;
        src.gwrite_with::<f32>(self.y, offset, ctx)?;
        src.gwrite_with::<f32>(self.z, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Vec3 {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;
        result += size_of_val(&self.x);
        result += size_of_val(&self.y);
        result += size_of_val(&self.z);
        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl ctx::TryFromCtx<'_, Endian> for Vec4 {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let x = src.gread_with(offset, ctx)?;
        let y = src.gread_with(offset, ctx)?;
        let z = src.gread_with(offset, ctx)?;
        let w = src.gread_with(offset, ctx)?;

        Ok((Vec4 { x, y, z, w }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Vec4 {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<f32>(self.x, offset, ctx)?;
        src.gwrite_with::<f32>(self.y, offset, ctx)?;
        src.gwrite_with::<f32>(self.z, offset, ctx)?;
        src.gwrite_with::<f32>(self.w, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Vec4 {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;
        result += size_of_val(&self.x);
        result += size_of_val(&self.y);
        result += size_of_val(&self.z);
        result += size_of_val(&self.w);
        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Color {
    b: f32,
    g: f32,
    r: f32,
}

impl ctx::TryFromCtx<'_, Endian> for Color {
    type Error = scroll::Error;

    fn try_from_ctx(src: &'_ [u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let b = src.gread_with(offset, ctx)?;
        let g = src.gread_with(offset, ctx)?;
        let r = src.gread_with(offset, ctx)?;

        Ok((Color { b, g, r }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Color {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<f32>(self.b, offset, ctx)?;
        src.gwrite_with::<f32>(self.g, offset, ctx)?;
        src.gwrite_with::<f32>(self.r, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Color {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;
        result += size_of_val(&self.b);
        result += size_of_val(&self.g);
        result += size_of_val(&self.r);
        result
    }
}
