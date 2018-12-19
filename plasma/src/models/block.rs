use super::Fr;

#[derive(Clone, SmartDefault)]
pub struct Block<T: Sized> {
    pub block_number:   u32,
    pub transactions:   Vec<T>,
    pub new_root_hash:  Fr,
}