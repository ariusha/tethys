use crate::{
    memory::{ProcessMemoryData, ThreadMemoryData},
    scheduler::{ProcessSchedulingData, ThreadSchedulingData},
    task::{ProcessTaskData, ThreadTaskData},
    tethys::{ProcessTethysData, ThreadTethysData},
};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use ostd::sync::RwMutex;
pub struct ThreadPriorityData {
    assigned: u64,
    propagated: u64,
}
pub struct ProcessPriorityData {
    assigned: u64,
    propagated: u64,
    inherited: u64,
}
pub struct Thread {
    pub active: bool,
    pub priorities: ThreadPriorityData,
    pub scheduling: ThreadSchedulingData,
    pub parent: Weak<Process>,
    pub tethys: ThreadTethysData,
    pub task: ThreadTaskData,
    pub memory: ThreadMemoryData,
}
pub struct Process {
    pub priorities: RwMutex<ProcessPriorityData>,
    pub scheduling: RwMutex<ProcessSchedulingData>,
    pub parent: Option<Weak<Process>>,
    pub threads: RwMutex<Vec<Arc<RwMutex<Thread>>>>,
    pub children: RwMutex<Vec<Arc<Process>>>,
    pub tethys: ProcessTethysData,
    pub task: ProcessTaskData,
    pub memory: ProcessMemoryData,
}
impl Default for ThreadPriorityData {
    fn default() -> Self {
        Self {
            assigned: 0,
            propagated: 0,
        }
    }
}
impl Default for ProcessPriorityData {
    fn default() -> Self {
        Self {
            assigned: 0,
            propagated: 0,
            inherited: 0,
        }
    }
}
impl<'a> Process {
    fn recalculate_priorities(self: &Self) {
        let priority_sum = self
            .children
            .read()
            .iter()
            .map(|child| child.priorities.read().assigned)
            .chain(
                self.threads
                    .read()
                    .iter()
                    .map(|thread| thread.read().priorities.assigned),
            )
            .map(|assigned| assigned as u128)
            .sum::<u128>();
        let redistribute_priorities = |assigned: u64, propagated: &mut u64| {
            *propagated = ((u64::MAX as u128 * assigned as u128) / priority_sum) as u64
        };
        for mut priorities in self
            .children
            .write()
            .iter()
            .map(|child| child.priorities.write())
        {
            redistribute_priorities(priorities.assigned, &mut priorities.propagated);
        }
        for mut thread in self.threads.write().iter().map(|thread| thread.write()) {
            redistribute_priorities(
                thread.priorities.assigned,
                &mut thread.priorities.propagated,
            );
        }
    }
    pub fn add_thread(self: &Arc<Self>) -> Arc<RwMutex<Thread>> {
        let thread = Arc::new(RwMutex::new(Thread {
            active: true,
            priorities: ThreadPriorityData::default(),
            scheduling: ThreadSchedulingData::default(),
            parent: Arc::downgrade(&self),
            tethys: ThreadTethysData::default(),
            task: ThreadTaskData::default(),
            memory: ThreadMemoryData::default(),
        }));
        self.threads.write().push(thread.clone());
        thread
    }
    pub fn add_child(self: &Arc<Self>) -> Arc<Process> {
        let mut children_write = self.children.write();
        let child = Arc::new(Process {
            priorities: RwMutex::new(ProcessPriorityData::default()),
            scheduling: RwMutex::new(ProcessSchedulingData::default()),
            parent: Some(Arc::downgrade(&self)),
            threads: RwMutex::new(Vec::new()),
            children: RwMutex::new(Vec::new()),
            tethys: ProcessTethysData::default(),
            task: ProcessTaskData::default(),
            memory: ProcessMemoryData::default(),
        });
        children_write.push(child.clone());
        child
    }
}
