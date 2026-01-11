use alloc::{sync::Arc, vec::Vec};
use ostd::{
    mm::{
        CachePolicy, FallibleVmRead, FallibleVmWrite, Frame, FrameAllocOptions, PAGE_SIZE,
        PageFlags, PageProperty, Vaddr, VmReader, VmSpace, VmWriter,
    },
    task::disable_preempt,
};
const KICKSTART_STACK_SIZE: usize = 64 * 1024;
const KICKSTART_STACK_TOP: Vaddr = 0x00007fffffffffff;
const USER_SPACE_MAX: u64 = 0x0000_7fff_ffff_ffff;
const KERNEL_SPACE_START: u64 = 0xffff_8000_0000_0000;
const ADDRESS_HOLE_START: u64 = 0x0000_8000_0000_0000;
pub struct Message(Vec<u8>);
impl Message {
    pub fn new(content: Vec<u8>) -> Message {
        Message(content)
    }
    pub fn len(self: &Self) -> u64 {
        self.0.len() as u64
    }
    pub fn take(self: Self) -> Vec<u8> {
        self.0
    }
    pub fn peek(self: &Self) -> &[u8] {
        self.0.as_slice()
    }
    pub fn peek_mut(self: &mut Self) -> &mut [u8] {
        self.0.as_mut_slice()
    }
}
pub struct ThreadMemoryData {}
pub struct ProcessMemoryData {
    pub vm_space: Arc<VmSpace>,
}
impl Default for ThreadMemoryData {
    fn default() -> Self {
        Self {}
    }
}
impl Default for ProcessMemoryData {
    fn default() -> Self {
        Self {
            vm_space: Arc::new(VmSpace::new()),
        }
    }
}
impl ProcessMemoryData {
    pub fn new_message(self: &Self, page: u64, count: u64) -> Option<Message> {
        let length = count as usize * PAGE_SIZE;
        let mut content: Vec<u8> = Vec::with_capacity(length);
        self.vm_space
            .reader((page as usize * PAGE_SIZE) as Vaddr, length)
            .ok()?
            .read_fallible(&mut VmWriter::from(content.as_mut_slice()))
            .ok()?;
        Some(Message::new(content))
    }
    pub fn accept_message(self: &Self, message: Message, page: u64) -> Option<Message> {
        if let Some(mut writer) = self
            .vm_space
            .writer(page as usize * PAGE_SIZE, message.len() as usize)
            .ok()
        {
            let content = message.take();
            match writer.write_fallible(&mut VmReader::from(content.as_slice())) {
                Ok(..) => None,
                Err(..) => Some(Message::new(content)),
            }
        } else {
            Some(message)
        }
    }
}
fn is_page_mapped(process_memory: &ProcessMemoryData, addr: usize) -> bool {
    let page_aligned_addr = addr & !(PAGE_SIZE as usize - 1);
    let preemption_guard = ostd::task::disable_preempt();
    match process_memory.vm_space.cursor_mut(
        &preemption_guard,
        &(page_aligned_addr..page_aligned_addr + PAGE_SIZE as usize),
    ) {
        Ok(_) => false,
        Err(_) => true,
    }
}
pub fn handle_map(process_memory: &ProcessMemoryData, page_offset: u64, page_count: u64) -> bool {
    if page_count == 0 {
        return true;
    }
    let start_addr = match page_offset.checked_mul(PAGE_SIZE as u64) {
        Some(addr) => addr,
        None => return false,
    };
    let range_size = match page_count.checked_mul(PAGE_SIZE as u64) {
        Some(size) => size,
        None => return false,
    };
    let end_addr = match start_addr.checked_add(range_size) {
        Some(addr) => addr,
        None => return false,
    };
    if start_addr >= KERNEL_SPACE_START || end_addr > KERNEL_SPACE_START {
        return false;
    }
    if (start_addr >= ADDRESS_HOLE_START && start_addr < KERNEL_SPACE_START)
        || (end_addr > ADDRESS_HOLE_START && end_addr <= KERNEL_SPACE_START)
    {
        return false;
    }
    if end_addr > USER_SPACE_MAX + 1 {
        return false;
    }
    let preemption_guard = disable_preempt();
    process_memory
        .vm_space
        .cursor_mut(&preemption_guard, &(start_addr as usize..end_addr as usize))
        .expect("failed to acquire process's address space during handling of map syscall!")
        .unmap((end_addr - start_addr) as usize);
    todo!();
    let mut current = start_addr as usize;
    let mut frames_needed = 0;
    while current < end_addr as usize {
        if !is_page_mapped(process_memory, current) {
            frames_needed += 1;
        }
        current += PAGE_SIZE as usize;
    }
    if frames_needed == 0 {
        return true;
    }
    let frames = match FrameAllocOptions::new().alloc_segment(frames_needed) {
        Ok(frames) => {
            let frames_vec: Vec<Frame<()>> = frames.collect();
            if frames_vec.len() != frames_needed {
                return false;
            }
            for frame in &frames_vec {
                todo!(); // zero it
            }
            frames_vec
        }
        Err(_) => return false,
    };
    let mut frame_iter = frames.into_iter();
    current = start_addr as usize;
    while current < end_addr as usize {
        if !is_page_mapped(process_memory, current) {
            let page_end = current + PAGE_SIZE as usize;
            match process_memory
                .vm_space
                .cursor_mut(&preemption_guard, &(current..page_end))
            {
                Ok(mut cursor) => {
                    if let Some(frame) = frame_iter.next() {
                        let map_properties =
                            PageProperty::new_user(PageFlags::RWX, CachePolicy::Writeback);
                        cursor.map(frame.into(), map_properties);
                    } else {
                        return false;
                    }
                }
                Err(_) => {}
            }
        }
        current += PAGE_SIZE as usize;
    }
    true
}
pub fn handle_unmap(process_memory: &ProcessMemoryData, page_offset: u64, page_count: u64) -> bool {
    if page_count == 0 {
        return true;
    }
    let start_addr = match page_offset.checked_mul(PAGE_SIZE as u64) {
        Some(addr) => addr,
        None => return false,
    };
    let range_size = match page_count.checked_mul(PAGE_SIZE as u64) {
        Some(size) => size,
        None => return false,
    };
    let end_addr = match start_addr.checked_add(range_size) {
        Some(addr) => addr,
        None => return false,
    };
    if start_addr >= KERNEL_SPACE_START || end_addr > KERNEL_SPACE_START {
        return false;
    }
    if (start_addr >= ADDRESS_HOLE_START && start_addr < KERNEL_SPACE_START)
        || (end_addr > ADDRESS_HOLE_START && end_addr <= KERNEL_SPACE_START)
    {
        return false;
    }
    if end_addr > USER_SPACE_MAX + 1 {
        return false;
    }
    let start_addr_usize = start_addr as usize;
    let end_addr_usize = end_addr as usize;
    let preemption_guard = disable_preempt();
    process_memory
        .vm_space
        .cursor_mut(&preemption_guard, &(start_addr_usize..end_addr_usize))
        .expect("failed to acquire process's address space during handling of unmap syscall!")
        .unmap((end_addr_usize - start_addr_usize) / PAGE_SIZE as usize);
    true
}
pub fn handle_copy(
    process_memory: &ProcessMemoryData,
    from_page: u64,
    to_page: u64,
    page_count: u64,
) -> bool {
    todo!()
}
pub fn handle_switch(
    process_memory: &ProcessMemoryData,
    from_page: u64,
    to_page: u64,
    page_count: u64,
) -> bool {
    todo!()
}
