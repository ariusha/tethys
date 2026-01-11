use core::sync::atomic::{AtomicU64, Ordering};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(pub u64);
pub struct TagAllocator(AtomicU64);
impl TagAllocator {
    pub fn new() -> TagAllocator {
        TagAllocator(AtomicU64::new(0))
    }
    pub fn allocate(self: &Self) -> Tag {
        Tag(self.0.fetch_add(1, Ordering::Relaxed))
    }
}
