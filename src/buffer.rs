use spin::Mutex;
use crate::{block_cache::BlockCahce, dev::BlockDevice};
use alloc::{collections::VecDeque, sync::Arc};

pub fn getblk(_id: usize, _dev: Arc<dyn BlockDevice>) -> Arc<Mutex<BlockCahce>> {
    todo!()
}

pub struct BufferManager {
    pub queue: VecDeque<(usize, Arc<Mutex<BlockCahce>>)>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    pub(crate) fn _cache_block(&mut self, _block_id: usize, _dev: Arc<dyn BlockDevice>) -> Arc<BlockCahce> {
        todo!()
    }
}

impl Default for BufferManager {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref BUFFER_MANAGER: Mutex<BufferManager>
        = Mutex::new(BufferManager::default());
}
