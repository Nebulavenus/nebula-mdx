use crate::chunks::{BytesTotalSize, Color, Node, Transform};
use crate::consts::{KLAC_TAG, KLAE_TAG, KLAI_TAG, KLAS_TAG, KLAV_TAG, KLBC_TAG, KLBI_TAG};
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

// TODO(nv): NOT TESTED! WRITE TESTS.

#[derive(PartialEq, Debug)]
pub struct LightChunk {
    pub chunk_size: u32,

    pub data: Vec<Light>,
}

calculate_chunk_size_impl!(LightChunk);

impl ctx::TryFromCtx<'_, Endian> for LightChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let light = src.gread_with::<Light>(offset, ctx)?;
            total_size += light.inclusive_size;
            data.push(light);
        }

        Ok((LightChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for LightChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for light in self.data {
            src.gwrite_with::<Light>(light, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for LightChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for light in &self.data {
            result += light.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Light {
    pub inclusive_size: u32,

    pub node: Node,

    pub light_type: u32, // 0: omni 1: directional 2: ambient
    pub attenuation_start: f32,
    pub attenuation_end: f32,
    pub color: Color,
    pub intensity: f32,
    pub ambient_color: Color,
    pub ambient_intensity: f32,

    pub attenuation_start_transform: Option<Transform<u32>>,
    pub attenuation_end_transform: Option<Transform<u32>>,

    pub color_transform: Option<Transform<Color>>,
    pub ambient_color_transform: Option<Transform<Color>>,

    pub intensity_transform: Option<Transform<f32>>,
    pub ambient_intensity_transform: Option<Transform<f32>>,
    pub visibility_transform: Option<Transform<f32>>,
}

impl ctx::TryFromCtx<'_, Endian> for Light {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        let node = src.gread_with::<Node>(offset, ctx)?;

        let light_type = src.gread_with::<u32>(offset, ctx)?;
        let attenuation_start = src.gread_with::<f32>(offset, ctx)?;
        let attenuation_end = src.gread_with::<f32>(offset, ctx)?;
        let color = src.gread_with::<Color>(offset, ctx)?;
        let intensity = src.gread_with::<f32>(offset, ctx)?;
        let ambient_color = src.gread_with::<Color>(offset, ctx)?;
        let ambient_intensity = src.gread_with::<f32>(offset, ctx)?;

        let mut light = Light {
            inclusive_size,
            node,
            light_type,
            attenuation_start,
            attenuation_end,
            color,
            intensity,
            ambient_color,
            ambient_intensity,
            attenuation_start_transform: None,
            attenuation_end_transform: None,
            color_transform: None,
            ambient_color_transform: None,
            intensity_transform: None,
            ambient_intensity_transform: None,
            visibility_transform: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx)?;

            match tag {
                KLAS_TAG => {
                    let ktas = src.gread_with(offset, ctx)?;
                    light.attenuation_start_transform = Some(ktas);
                }
                KLAE_TAG => {
                    let ktae = src.gread_with(offset, ctx)?;
                    light.attenuation_end_transform = Some(ktae);
                }
                KLAC_TAG => {
                    let klac = src.gread_with(offset, ctx)?;
                    light.color_transform = Some(klac);
                }
                KLAI_TAG => {
                    let klai = src.gread_with(offset, ctx)?;
                    light.intensity_transform = Some(klai);
                }
                KLBC_TAG => {
                    let klbc = src.gread_with(offset, ctx)?;
                    light.ambient_color_transform = Some(klbc);
                }
                KLBI_TAG => {
                    let klbi = src.gread_with(offset, ctx)?;
                    light.ambient_intensity_transform = Some(klbi);
                }
                KLAV_TAG => {
                    let klav = src.gread_with(offset, ctx)?;
                    light.visibility_transform = Some(klav);
                }
                _ => unreachable!(),
            }
        }

        Ok((light, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Light {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;

        src.gwrite_with::<Node>(self.node, offset, ctx)?;

        src.gwrite_with::<u32>(self.light_type, offset, ctx)?;
        src.gwrite_with::<f32>(self.attenuation_start, offset, ctx)?;
        src.gwrite_with::<f32>(self.attenuation_end, offset, ctx)?;
        src.gwrite_with::<Color>(self.color, offset, ctx)?;
        src.gwrite_with::<f32>(self.intensity, offset, ctx)?;
        src.gwrite_with::<Color>(self.ambient_color, offset, ctx)?;
        src.gwrite_with::<f32>(self.ambient_intensity, offset, ctx)?;

        if self.attenuation_start_transform.is_some() {
            src.gwrite_with::<u32>(KLAS_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<u32>>(
                self.attenuation_start_transform.unwrap(),
                offset,
                ctx,
            )?;
        }
        if self.attenuation_end_transform.is_some() {
            src.gwrite_with::<u32>(KLAE_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<u32>>(
                self.attenuation_end_transform.unwrap(),
                offset,
                ctx,
            )?;
        }
        if self.color_transform.is_some() {
            src.gwrite_with::<u32>(KLAC_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Color>>(self.color_transform.unwrap(), offset, ctx)?;
        }
        if self.intensity_transform.is_some() {
            src.gwrite_with::<u32>(KLAI_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<f32>>(self.intensity_transform.unwrap(), offset, ctx)?;
        }
        if self.ambient_color_transform.is_some() {
            src.gwrite_with::<u32>(KLBC_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<Color>>(
                self.ambient_color_transform.unwrap(),
                offset,
                ctx,
            )?;
        }
        if self.ambient_intensity_transform.is_some() {
            src.gwrite_with::<u32>(KLBI_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<f32>>(
                self.ambient_intensity_transform.unwrap(),
                offset,
                ctx,
            )?;
        }
        if self.visibility_transform.is_some() {
            src.gwrite_with::<u32>(KLAV_TAG, offset, ctx)?;
            src.gwrite_with::<Transform<f32>>(self.visibility_transform.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Light {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        result += self.node.total_bytes_size();

        result += size_of_val(&self.light_type);
        result += size_of_val(&self.attenuation_start);
        result += size_of_val(&self.attenuation_end);
        result += self.color.total_bytes_size();
        result += size_of_val(&self.intensity);
        result += self.ambient_color.total_bytes_size();
        result += size_of_val(&self.ambient_intensity);

        if self.attenuation_start_transform.is_some() {
            result += 4;
            result += self
                .attenuation_start_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.attenuation_end_transform.is_some() {
            result += 4;
            result += self
                .attenuation_end_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.color_transform.is_some() {
            result += 4;
            result += self.color_transform.as_ref().unwrap().total_bytes_size();
        }
        if self.intensity_transform.is_some() {
            result += 4;
            result += self
                .intensity_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.ambient_color_transform.is_some() {
            result += 4;
            result += self
                .ambient_color_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.ambient_intensity_transform.is_some() {
            result += 4;
            result += self
                .ambient_intensity_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }
        if self.visibility_transform.is_some() {
            result += 4;
            result += self
                .visibility_transform
                .as_ref()
                .unwrap()
                .total_bytes_size();
        }

        result
    }
}
