use crate::{interface::handle_syscall, process::Thread};
use alloc::sync::Arc;
use ostd::{
    arch::cpu::context::{FpuContext, UserContext},
    mm::Vaddr,
    sync::{RwMutex, WaitQueue},
    task::Task,
    user::{ReturnReason, UserMode},
};
pub struct PanicVectors {
    emergency: Vaddr,
    divide: Vaddr,
    debug: Vaddr,
    breakpoint: Vaddr,
    overflow: Vaddr,
    bound: Vaddr,
    opcode: Vaddr,
    device: Vaddr,
    double: Vaddr,
    stack: Vaddr,
    protection: Vaddr,
    page: Vaddr,
    floating: Vaddr,
    simd: Vaddr,
    control: Vaddr,
    security: Vaddr,
    invalid_syscall: Vaddr,
}
pub struct ThreadTaskData {
    pub panic_vectors: PanicVectors,
    pub user_context: Option<UserContext>,
    pub fpu_context: Option<FpuContext>,
    pub is_active: bool,
    pub inactivity_queue: WaitQueue,
}
pub struct ProcessTaskData {}
impl Default for ThreadTaskData {
    fn default() -> Self {
        Self {
            panic_vectors: PanicVectors {
                emergency: 0,
                divide: 0,
                debug: 0,
                breakpoint: 0,
                overflow: 0,
                bound: 0,
                opcode: 0,
                device: 0,
                double: 0,
                stack: 0,
                protection: 0,
                page: 0,
                floating: 0,
                simd: 0,
                control: 0,
                security: 0,
                invalid_syscall: 0,
            },
            user_context: None,
            fpu_context: None,
            is_active: false,
            inactivity_queue: WaitQueue::new(),
        }
    }
}
impl Default for ProcessTaskData {
    fn default() -> Self {
        Self {}
    }
}
pub fn user_task() {
    let task = Task::current().expect("task handler function executed from bootstrap context!");
    let thread = task
        .data()
        .downcast_ref::<Arc<RwMutex<Thread>>>()
        .expect("task handler function failed to acquire task data!");
    thread
        .read()
        .parent
        .upgrade()
        .expect("thread orphaned from parent process during address space activation!")
        .memory
        .vm_space
        .activate();
    loop {
        let mut user_mode = UserMode::new(
            thread
                .read()
                .task
                .user_context
                .as_ref()
                .expect("executing thread has no execution context!")
                .clone(),
        );
        let return_reason = user_mode.execute(|| false);
        let process = thread
            .read()
            .parent
            .upgrade()
            .expect("executing thread orphaned from parent process!");
        let context = user_mode.context();
        if let Some((should_exit, new_registers)) = match return_reason {
            ReturnReason::UserSyscall => Some(handle_syscall(
                [
                    context.rax(),
                    context.rbx(),
                    context.rcx(),
                    context.rdx(),
                    context.rsi(),
                ]
                .map(|as_usize| as_usize as u64),
                thread.as_ref(),
                &process,
            )),
            ReturnReason::UserException => todo!(),
            ReturnReason::KernelEvent => panic!("unhandled kernel event in task handler!"),
        } {
            if should_exit {
                break;
            }
            let context_mut = user_mode.context_mut();
            if let Some(Some(value)) = new_registers.get(0) {
                context_mut.set_rax(value.clone() as usize);
            }
            if let Some(Some(value)) = new_registers.get(0) {
                context_mut.set_rbx(value.clone() as usize);
            }
            if let Some(Some(value)) = new_registers.get(0) {
                context_mut.set_rcx(value.clone() as usize);
            }
            if let Some(Some(value)) = new_registers.get(0) {
                context_mut.set_rdx(value.clone() as usize);
            }
            if let Some(Some(value)) = new_registers.get(0) {
                context_mut.set_rsi(value.clone() as usize);
            }
        }
    }
}
