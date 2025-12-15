use crate::{
    acpi::PROCESSOR_COUNT,
    gdt::{self, Selectors},
    println,
    proc::{Process, Thread},
    scheduler::ProcessorScheduler,
};
use alloc::{
    boxed::Box,
    collections::{binary_heap::BinaryHeap, vec_deque::VecDeque},
    sync::{Arc, Weak},
    vec::Vec,
};
use spinning_top::RwSpinlock;
use x86_64::structures::gdt::GlobalDescriptorTable;
#[repr(C, align(64))]
pub struct ProcessorData {
    pub gdt_selectors: (&'static GlobalDescriptorTable, Selectors),
    pub scheduler: ProcessorScheduler,
}
#[repr(C, packed)]
pub struct ProcessorDataPointer(*const &'static mut RwSpinlock<ProcessorData>);
unsafe impl Send for ProcessorDataPointer {}
unsafe impl Sync for ProcessorDataPointer {}
pub static PROCESSOR_DATA_VEC: RwSpinlock<Vec<&'static mut RwSpinlock<ProcessorData>>> =
    RwSpinlock::new(Vec::new());
pub static PROCESSOR_DATA_POINTERS: RwSpinlock<Vec<ProcessorDataPointer>> =
    RwSpinlock::new(Vec::new());
pub fn initialise(_boot_info: &mut bootloader_api::BootInfo) {
    let mut processor_data = PROCESSOR_DATA_VEC.write();
    processor_data.append(
        &mut (0..PROCESSOR_COUNT
            .read()
            .expect("processors not counted before per-processor initialisation!"))
            .map(|index| {
                let gdt_selectors = gdt::new(index);
                Box::leak(Box::new(RwSpinlock::new(ProcessorData {
                    gdt_selectors: gdt_selectors,
                    scheduler: ProcessorScheduler::new(),
                })))
            })
            .collect::<Vec<&'static mut RwSpinlock<ProcessorData>>>(),
    );
    let mut processor_data_ptrs = PROCESSOR_DATA_POINTERS.write();
    for per_processor in processor_data.iter_mut() {
        processor_data_ptrs.push(ProcessorDataPointer(per_processor as *const &'static mut RwSpinlock<ProcessorData>));
    }
    println!(
        "initialised processor data for {} processors...",
        processor_data.len()
    );
    let bootstrap_data = processor_data
        .get(0)
        .expect("bootstrap processor could not find processor data!")
        .read();
    unsafe { gdt::load(&bootstrap_data.gdt_selectors) };
}
