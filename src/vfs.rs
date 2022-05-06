use core::any::Any;

use alloc::sync::Arc;

pub enum FsError {
    InvalidParam,
}

pub type Result<T> = core::result::Result<T, FsError>;

pub trait Inode: Any + Sync + Send {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize>;

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize>;

    fn metadata(&self) -> Result<Metadata>;

    fn link(&self, name: &str, other: &Arc<dyn Inode>) -> Result<()>;

    fn unlink(&self, name: &str) -> Result<()>;

    fn ioctl(&self, cmd: u32, data: usize) -> Result<usize>;

    fn mmap(&self, area: MMapArea) -> Result<()>;
}

pub trait FileSystem: Any + Sync + Send {
    fn sync(&self) -> Result<()>;

    fn root_inode(&self) -> Arc<dyn Inode>;

    fn info(&self) -> FsInfo;
}

pub enum FileType {
    File,
    Directory,
    Symlink,
    NamedPipe,
    CharDevice,
    BlockDevice,
    Socket,
}

pub struct Timespec {
    pub sec: i64,
    pub nsec: i32,
}

/// Metadata of Inode
pub struct Metadata {
    /// Device ID
    pub dev: usize, // (major << 8) | minor
    /// Inode number
    pub inode: usize,
    /// Size in bytes
    pub size: usize,
    pub blk_size: usize,
    /// Size in blocks
    pub blocks: usize,
    /// Time of last access
    pub atime: Timespec,
    /// Time of last modification
    pub mtime: Timespec,
    /// Time of last change
    pub ctime: Timespec,
    /// Type of file
    pub type_: FileType,
    /// Permission
    pub mode: u16,
    pub nlinks: usize,
    /// User ID
    pub uid: usize,
    /// Group ID
    pub gid: usize,
    /// Raw device id
    /// e.g. /dev/null: makedev(0x1, 0x3)
    pub rdev: usize, // (major << 8) | minor
}

/// Metadata of FileSystem
pub struct FsInfo {
    /// File system block size
    pub bsize: usize,
    /// Fundamental file system block size
    pub frsize: usize,
    /// Total number of blocks on file system in units of `frsize`
    pub blocks: usize,
    /// Total number of free blocks
    pub bfree: usize,
    /// Number of free blocks available to non-privileged process
    pub bavail: usize,
    /// Total number of file serial numbers
    pub files: usize,
    /// Total number of free file serial numbers
    pub ffree: usize,
    /// Maximum filename length
    pub namemax: usize,
}

pub struct MMapArea {
    /// Start virtual address
    pub start_vaddr: usize,
    /// End virtual address
    pub end_vaddr: usize,
    /// Access permissions
    pub prot: usize,
    /// Flags
    pub flags: usize,
    /// Offset from the file in bytes
    pub offset: usize,
}

pub fn make_rdev(major: usize, minor: usize) -> usize {
    ((major & 0xfff) << 8) | (minor & 0xff)
}
