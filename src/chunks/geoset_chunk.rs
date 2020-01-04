use crate::chunks::BytesTotalSize;
use scroll::{ctx, Endian, Pread, Pwrite, Error};
use crate::chunks::Extent;
use std::mem::size_of_val;

#[derive(PartialEq, Debug)]
pub struct GeosetChunk {
    pub chunk_size: u32,

    pub data: Vec<Geoset>,
}

calculate_chunk_size_impl!(GeosetChunk);

impl ctx::TryFromCtx<'_, Endian> for GeosetChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let geoset = src.gread_with::<Geoset>(offset, ctx)?;
            //total_size += bone.node.inclusive_size + 4 + 4;
            //total_size += geoset.total_bytes_size() as u32;
            total_size += geoset.inclusive_size;
            assert_eq!(geoset.inclusive_size, geoset.total_bytes_size() as u32);
            data.push(geoset);
        }

        Ok((GeosetChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for GeosetChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for geoset in self.data {
            src.gwrite_with::<Geoset>(geoset, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for GeosetChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for geoset in &self.data {
            result += geoset.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Geoset {
    pub inclusive_size: u32,

    pub vertex_count: u32, // VRTX
    pub vertex_positions: Vec<VertexPosition>,

    pub normal_count: u32, // NRMS
    pub vertex_normals: Vec<VertexNormal>,

    pub face_type_groups_count: u32, // PTYP
    pub face_type_groups: Vec<FaceTypeGroup>,

    pub face_groups_count: u32, // PCNT
    pub face_groups: Vec<FaceGroup>,

    pub faces_count: u32, // PVTX
    pub faces: Vec<Face>,

    pub vertex_groups_count: u32, // GNDX
    pub vertex_groups: Vec<VertexGroup>,

    pub matrix_groups_count: u32, // MTGC
    pub matrix_groups: Vec<MatrixGroup>,

    pub matrix_indexes_count: u32, // MATS
    pub matrix_indexes: Vec<MatrixIndex>,

    pub material_id: u32,
    pub selection_group: u32,
    pub selection_type: u32, // 0 - None | 4 - Unselectable

    pub extent: Extent,
    pub extents_count: u32,
    pub extent_sequences: Vec<Extent>,

    pub texture_coordinate_sets_count: u32, // UVAS - UVBS inside
    pub texture_coordinate_sets: Vec<TextureCoordinateSet>,
}

impl ctx::TryFromCtx<'_, Endian> for Geoset {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        let check_for_tag = |offset: &mut usize, expect_tag: &str| -> Result<_, Self::Error> {
            info!("TagOffset: {}", &offset);
            let mut tag_offset = offset.clone();
            let tag_buffer = (0..4)
                .map(|_| src.gread::<u8>(&mut tag_offset).unwrap())
                .collect::<Vec<u8>>();
            let tag_name = String::from_utf8(tag_buffer).unwrap_or("NOTAG".to_string());
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            info!("TagHex: {}", format!("{:X}", &tag));
            info!("TagDec: {}", &tag);
            info!("TagName: {}", &tag_name);

            if tag_name != expect_tag {
                return Err(Error::Custom(
                    format!("Geoset format is not correct. Expected {} - Found {}", expect_tag, tag_name)
                ));
            }
            Ok(())
        };

        // VRTX
        check_for_tag(offset, "VRTX")?;

        let vertex_count = src.gread_with::<u32>(offset, ctx)?;
        let mut vertex_positions = Vec::new();
        for _ in 0..vertex_count {
            let value = src.gread_with::<VertexPosition>(offset, ctx)?;
            vertex_positions.push(value);
        }

        // NRMS
        check_for_tag(offset, "NRMS")?;

        let normal_count = src.gread_with::<u32>(offset, ctx)?;
        let mut vertex_normals = Vec::new();
        for _ in 0..normal_count {
            let value = src.gread_with::<VertexNormal>(offset, ctx)?;
            vertex_normals.push(value);
        }

        // PTYP
        check_for_tag(offset, "PTYP")?;

        let face_type_groups_count = src.gread_with::<u32>(offset, ctx)?;
        let mut face_type_groups = Vec::new();
        for _ in 0..face_type_groups_count {
            let value = src.gread_with::<FaceTypeGroup>(offset, ctx)?;
            face_type_groups.push(value);
        }

        // PCNT
        check_for_tag(offset, "PCNT")?;

        let face_groups_count = src.gread_with::<u32>(offset, ctx)?;
        let mut face_groups = Vec::new();
        for _ in 0..face_groups_count {
            let value = src.gread_with::<FaceGroup>(offset, ctx)?;
            face_groups.push(value);
        }

        // PVTX
        check_for_tag(offset, "PVTX")?;

        let faces_count = src.gread_with::<u32>(offset, ctx)?;
        let mut faces = Vec::new();
        for _ in 0..faces_count {
            let value = src.gread_with::<Face>(offset, ctx)?;
            faces.push(value);
        }

        // GNDX
        check_for_tag(offset, "GNDX")?;

        let vertex_groups_count = src.gread_with::<u32>(offset, ctx)?;
        let mut vertex_groups = Vec::new();
        for _ in 0..vertex_groups_count {
            let value = src.gread_with::<VertexGroup>(offset, ctx)?;
            vertex_groups.push(value);
        }

        // MTGC
        check_for_tag(offset, "MTGC")?;

        let matrix_groups_count = src.gread_with::<u32>(offset, ctx)?;
        let mut matrix_groups = Vec::new();
        for _ in 0..matrix_groups_count {
            let value = src.gread_with::<MatrixGroup>(offset, ctx)?;
            matrix_groups.push(value);
        }

        // MATS
        check_for_tag(offset, "MATS")?;

        let matrix_indexes_count = src.gread_with::<u32>(offset, ctx)?;
        let mut matrix_indexes = Vec::new();
        for _ in 0..matrix_indexes_count {
            let value = src.gread_with::<MatrixIndex>(offset, ctx)?;
            matrix_indexes.push(value);
        }

        let material_id = src.gread_with::<u32>(offset, ctx)?;
        let selection_group = src.gread_with::<u32>(offset, ctx)?;
        let selection_type = src.gread_with::<u32>(offset, ctx)?;

        let extent = src.gread_with::<Extent>(offset, ctx)?;
        let extents_count = src.gread_with::<u32>(offset, ctx)?;
        let mut extent_sequences = Vec::new();
        for _ in 0..extents_count {
            let value = src.gread_with::<Extent>(offset, ctx)?;
            extent_sequences.push(value);
        }

        // UVAS | UVBS
        check_for_tag(offset, "UVAS")?;

        let texture_coordinate_sets_count = src.gread_with::<u32>(offset, ctx)?;
        let mut texture_coordinate_sets = Vec::new();
        for _ in 0..texture_coordinate_sets_count {
            check_for_tag(offset, "UVBS")?;

            let value = src.gread_with::<TextureCoordinateSet>(offset, ctx)?;
            texture_coordinate_sets.push(value);
        }

        Ok((Geoset {
            inclusive_size,
            vertex_count,
            vertex_positions,
            normal_count,
            vertex_normals,
            face_type_groups_count,
            face_type_groups,
            face_groups_count,
            face_groups,
            faces_count,
            faces,
            vertex_groups_count,
            vertex_groups,
            matrix_groups_count,
            matrix_groups,
            matrix_indexes_count,
            matrix_indexes,
            material_id,
            selection_group,
            selection_type,
            extent,
            extents_count,
            extent_sequences,
            texture_coordinate_sets_count,
            texture_coordinate_sets,
        }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Geoset {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        //src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        //src.gwrite_with::<&[u8]>(self.bytes.as_slice(), offset, ())?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Geoset {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        result += 4; // VRTX
        result += size_of_val(&self.vertex_count);
        for vp in &self.vertex_positions {
            result += vp.total_bytes_size();
        }

        result += 4; // NRMS
        result += size_of_val(&self.normal_count);
        for vn in &self.vertex_normals {
            result += vn.total_bytes_size();
        }

        result += 4; // PTYP
        result += size_of_val(&self.face_type_groups_count);
        for ftg in &self.face_type_groups {
            result += ftg.total_bytes_size();
        }

        result += 4; // PCNT
        result += size_of_val(&self.face_groups_count);
        for fg in &self.face_groups {
            result += fg.total_bytes_size();
        }

        result += 4; // PVTX
        result += size_of_val(&self.faces_count);
        for f in &self.faces {
            result += f.total_bytes_size();
        }

        result += 4; // GNDX
        result += size_of_val(&self.vertex_groups_count);
        for vg in &self.vertex_groups {
            result += vg.total_bytes_size();
        }

        result += 4; // MTGC
        result += size_of_val(&self.matrix_groups_count);
        for mg in &self.matrix_groups {
            result += mg.total_bytes_size();
        }

        result += 4; // MATS
        result += size_of_val(&self.matrix_indexes_count);
        for mi in &self.matrix_indexes {
            result += mi.total_bytes_size();
        }

        result += size_of_val(&self.material_id);
        result += size_of_val(&self.selection_group);
        result += size_of_val(&self.selection_type);

        result += size_of_val(&self.extent);
        result += size_of_val(&self.extents_count);
        for extent in &self.extent_sequences {
            result += extent.total_bytes_size();
        }

        result += 4; // UVAS
        result += size_of_val(&self.texture_coordinate_sets_count);
        for tcs in &self.texture_coordinate_sets {
            result += 4; // UVBS
            result += tcs.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct VertexPosition {
    pub position: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for VertexPosition {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let position = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((VertexPosition { position }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for VertexPosition {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        for id in 0..3 {
            src.gwrite_with::<f32>(self.position[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for VertexPosition {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.position);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct VertexNormal {
    pub normal: [f32; 3],
}

impl ctx::TryFromCtx<'_, Endian> for VertexNormal {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let normal = [
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
            src.gread_with::<f32>(offset, ctx)?,
        ];

        Ok((VertexNormal { normal }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for VertexNormal {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        for id in 0..3 {
            src.gwrite_with::<f32>(self.normal[id], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for VertexNormal {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.normal);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct FaceTypeGroup {
    pub face_type: u32, // always be 4 - triangle list
}

impl ctx::TryFromCtx<'_, Endian> for FaceTypeGroup {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let face_type = src.gread_with::<u32>(offset, ctx)?;

        Ok((FaceTypeGroup { face_type }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for FaceTypeGroup {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.face_type, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for FaceTypeGroup {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.face_type);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct FaceGroup {
    pub number_of_indexes: u32,
}

impl ctx::TryFromCtx<'_, Endian> for FaceGroup {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let number_of_indexes= src.gread_with::<u32>(offset, ctx)?;

        Ok((FaceGroup { number_of_indexes }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for FaceGroup {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.number_of_indexes, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for FaceGroup {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.number_of_indexes);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Face {
    pub index1: u16,
    pub index2: u16,
    pub index3: u16,
}

impl ctx::TryFromCtx<'_, Endian> for Face {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let index1 = src.gread_with::<u16>(offset, ctx)?;
        let index2 = src.gread_with::<u16>(offset, ctx)?;
        let index3 = src.gread_with::<u16>(offset, ctx)?;

        Ok((Face { index1, index2, index3 }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Face {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u16>(self.index1, offset, ctx)?;
        src.gwrite_with::<u16>(self.index2, offset, ctx)?;
        src.gwrite_with::<u16>(self.index3, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for Face {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.index1);
        result += size_of_val(&self.index2);
        result += size_of_val(&self.index3);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct VertexGroup {
    pub matrix_group: u8,
}

impl ctx::TryFromCtx<'_, Endian> for VertexGroup {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let matrix_group = src.gread_with::<u8>(offset, ctx)?;

        Ok((VertexGroup { matrix_group }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for VertexGroup {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u8>(self.matrix_group, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for VertexGroup {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.matrix_group);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct MatrixGroup {
    pub matrix_group_size: u32,
}

impl ctx::TryFromCtx<'_, Endian> for MatrixGroup {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let matrix_group_size = src.gread_with::<u32>(offset, ctx)?;

        Ok((MatrixGroup { matrix_group_size }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for MatrixGroup {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.matrix_group_size, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for MatrixGroup {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.matrix_group_size);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct MatrixIndex {
    pub matrix_index: u32,
}

impl ctx::TryFromCtx<'_, Endian> for MatrixIndex {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let matrix_index = src.gread_with::<u32>(offset, ctx)?;

        Ok((MatrixIndex { matrix_index }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for MatrixIndex {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.matrix_index, offset, ctx)?;

        Ok(*offset)
    }
}

impl BytesTotalSize for MatrixIndex {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.matrix_index);

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct TextureCoordinateSet {
    pub count: u32, // UVBS
    pub texture_coordinates: Vec<[f32; 2]>,
}

impl ctx::TryFromCtx<'_, Endian> for TextureCoordinateSet {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let count = src.gread_with::<u32>(offset, ctx)?;

        let mut texture_coordinates = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let texture_coordinate = [
                src.gread_with::<f32>(offset, ctx)?,
                src.gread_with::<f32>(offset, ctx)?,
            ];
            texture_coordinates.push(texture_coordinate);
        }
        assert_eq!(count as usize, texture_coordinates.len());
        assert_eq!(texture_coordinates.capacity(), texture_coordinates.len());

        Ok((TextureCoordinateSet { count, texture_coordinates }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for TextureCoordinateSet {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.count, offset, ctx)?;

        for tc in self.texture_coordinates {
            src.gwrite_with::<f32>(tc[0], offset, ctx)?;
            src.gwrite_with::<f32>(tc[1], offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for TextureCoordinateSet {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.count);
        for tc in &self.texture_coordinates {
            result += size_of_val(tc);
        }

        result
    }
}
