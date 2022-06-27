use core::any::Any;

/// Trait for block devices
/// which reads and writes data in the unit of blocks
pub trait BlockDevice : Send + Sync + Any {
    fn read_block(&self, block_id: usize, buf: &mut [u8]);
    fn write_block(&self, block_id: usize, buf: &[u8]);
}
