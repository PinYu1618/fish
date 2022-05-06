pub const BLOCK_SIZE: usize = 512;

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceError;

pub type Result<T> = core::result::Result<T, DeviceError>;

pub type BlockId = usize;

pub trait Device: Send + Sync {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize>;
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize>;
    fn sync(&self) -> Result<()>;
}

pub trait BlockDevice: Send + Sync {
    /// Read a new block from block device to buffer.
    fn read_at(&self, block_id: BlockId, buf: &mut [u8]) -> Result<usize>;
    /// Write block buffer content to block device.
    fn write_at(&self, block_id: BlockId, buf: &[u8]) -> Result<usize>;
    fn sync(&self) -> Result<()>;
}
