use crate::chunks::{AttachmentVisibility, BytesTotalSize, Node};
use crate::consts::KATV_TAG;
use scroll::{ctx, Endian, Pread, Pwrite};
use std::mem::size_of_val;

// TODO(nv): NOT TESTED! WRITE TESTS.

#[derive(PartialEq, Debug)]
pub struct AttachmentChunk {
    pub chunk_size: u32,

    pub data: Vec<Attachment>,
}

calculate_chunk_size_impl!(AttachmentChunk);

impl ctx::TryFromCtx<'_, Endian> for AttachmentChunk {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let chunk_size = src.gread_with::<u32>(offset, ctx)?;

        let mut data = Vec::new();
        let mut total_size = 0u32;
        while total_size < chunk_size {
            let attachment = src.gread_with::<Attachment>(offset, ctx)?;
            total_size += attachment.inclusive_size;
            assert_eq!(
                attachment.inclusive_size,
                attachment.total_bytes_size() as u32
            );
            data.push(attachment);
        }

        Ok((AttachmentChunk { chunk_size, data }, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for AttachmentChunk {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.chunk_size, offset, ctx)?;

        for geoset in self.data {
            src.gwrite_with::<Attachment>(geoset, offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for AttachmentChunk {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.chunk_size);

        for attachment in &self.data {
            result += attachment.total_bytes_size();
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Attachment {
    pub inclusive_size: u32,

    pub node: Node,
    // max length 260
    pub path: String,
    pub attachment_id: u32,

    pub visibility: Option<AttachmentVisibility>,
}

impl ctx::TryFromCtx<'_, Endian> for Attachment {
    type Error = scroll::Error;

    fn try_from_ctx(src: &[u8], ctx: Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let inclusive_size = src.gread_with::<u32>(offset, ctx)?;

        let node = src.gread_with::<Node>(offset, ctx)?;

        let max_name_len = 260usize;
        let path = src.gread::<&str>(&mut offset.clone())?.to_string();
        *offset += max_name_len;

        let attachment_id = src.gread_with::<u32>(offset, ctx)?;

        let mut attachment = Attachment {
            inclusive_size,
            node,
            path,
            attachment_id,
            visibility: None,
        };

        while (*offset as u32) < inclusive_size {
            let tag = src.gread_with::<u32>(offset, ctx).unwrap();

            match tag {
                KATV_TAG => {
                    let visibility = src.gread_with::<AttachmentVisibility>(offset, ctx)?;
                    attachment.visibility = Some(visibility);
                }
                _ => unreachable!(),
            }
        }

        Ok((attachment, *offset))
    }
}

impl ctx::TryIntoCtx<Endian> for Attachment {
    type Error = scroll::Error;

    fn try_into_ctx(self, src: &mut [u8], ctx: Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        src.gwrite_with::<u32>(self.inclusive_size, offset, ctx)?;
        src.gwrite_with::<Node>(self.node, offset, ctx)?;

        // String has fixed size
        let max_path_len = 260usize;
        let null_offset = &mut offset.clone();
        for _ in 0..max_path_len {
            src.gwrite_with::<u8>(0x0, null_offset, ctx)?;
        }
        src.gwrite_with::<&str>(self.path.as_ref(), &mut offset.clone(), ())?;
        *offset += max_path_len;

        src.gwrite_with::<u32>(self.attachment_id, offset, ctx)?;

        if self.visibility.is_some() {
            src.gwrite_with::<u32>(KATV_TAG, offset, ctx)?;
            src.gwrite_with::<AttachmentVisibility>(self.visibility.unwrap(), offset, ctx)?;
        }

        Ok(*offset)
    }
}

impl BytesTotalSize for Attachment {
    fn total_bytes_size(&self) -> usize {
        let mut result = 0usize;

        result += size_of_val(&self.inclusive_size);

        result += self.node.total_bytes_size();

        let max_path_len = 260usize;
        result += max_path_len;

        result += size_of_val(&self.attachment_id);

        if self.visibility.is_some() {
            result += 4;
            result += self.visibility.as_ref().unwrap().total_bytes_size();
        }

        result
    }
}
