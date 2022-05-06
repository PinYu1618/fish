use crate::{dev::BLOCK_SIZE, dev::BlockDevice};
use alloc::sync::Arc;

pub struct BlockCahce {
    pub content: [u8; BLOCK_SIZE],
    pub block_id: usize,
    pub block_dev: Arc<dyn BlockDevice>,
    pub modified: bool,
}

impl BlockCahce {
    pub fn block_id(&self) -> usize {
        self.block_id
    }

    pub fn content(&self) -> &[u8; BLOCK_SIZE] {
        &self.content
    }

    pub fn new(block_id: usize, dev: Arc<dyn BlockDevice>) -> Self {
        let mut buf = [0_u8; BLOCK_SIZE];
        //dev.block_read(block_id, &mut buf);
        Self {
            content: buf,
            block_id,
            block_dev: dev,
            modified: false,
        }
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            //self.block_dev.block_write(self.block_id(), self.content());
        }
    }

    pub fn address_of_offset(&self, offset: usize) -> usize {
        &self.content()[offset] as *const _ as usize
    }
}

impl Drop for BlockCahce {
    fn drop(&mut self) {
        self.sync()
    }
}

pub fn bread() -> BlockCahce {
    todo!()
}

pub fn breada() -> BlockCahce {
    todo!()
}

pub fn bread_page() -> BlockCahce {
    todo!()
}
