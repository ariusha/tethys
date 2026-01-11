use crate::{
    exchange::{QueryResult, Requestor, Responder, exchange},
    memory::Message,
    serialise::request_drop,
    state::State,
    tag::{Tag, TagAllocator},
};
use alloc::{
    boxed::Box,
    collections::vec_deque::VecDeque,
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use ostd::sync::{RwMutex, WaitQueue};
pub const PATH_DELIMITER: char = '/';
pub fn clean_path(path: &str) -> String {
    path.split(PATH_DELIMITER)
        .filter(|segment| segment.len() > 0)
        .collect()
}
pub struct Server {
    pub bind_table: RwMutex<BindTable>,
    pub queue: RwMutex<VecDeque<Responder>>,
    pub waiting: RwMutex<Vec<(Tag, Responder)>>,
    pub wait_queue: Arc<WaitQueue>,
    pub tag_allocator: TagAllocator,
}
pub struct Descriptor {
    pub server: Weak<Server>,
    pub path: String,
    pub mask: State,
    pub tag: Tag,
    pub list_head: u64,
}
impl Drop for Descriptor {
    fn drop(self: &mut Self) {
        let (requestor, responder) = exchange(
            request_drop(self.tag.0),
            Box::new(|requestor| requestor.accept()),
            Box::new(|responder, response| responder.respond(response)),
        );
        self.server
            .upgrade()
            .map(|server| server.request(responder));
        requestor.forget();
    }
}
pub struct BindTable {
    before: Vec<(String, Descriptor, State)>,
    after: Vec<(String, Descriptor, State)>,
}
impl BindTable {
    pub fn new() -> BindTable {
        BindTable {
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}
pub struct ThreadTethysData {}
pub struct ProcessTethysData {
    pub messages: RwMutex<Vec<(Tag, Requestor)>>,
    pub servers: RwMutex<Vec<(Tag, Arc<Server>)>>,
    pub descriptors: RwMutex<Vec<RwMutex<Descriptor>>>,
    pub tag_allocator: TagAllocator,
}
impl Default for ThreadTethysData {
    fn default() -> Self {
        Self {}
    }
}
impl Default for ProcessTethysData {
    fn default() -> Self {
        Self {
            messages: RwMutex::new(Vec::new()),
            servers: RwMutex::new(Vec::new()),
            descriptors: RwMutex::new(Vec::new()),
            tag_allocator: TagAllocator::new(),
        }
    }
}
impl ProcessTethysData {
    pub fn add_server(self: &Self) -> Arc<Server> {
        let server_arc = Arc::new(Server {
            bind_table: RwMutex::new(BindTable::new()),
            queue: RwMutex::new(VecDeque::new()),
            waiting: RwMutex::new(Vec::new()),
            wait_queue: Arc::new(WaitQueue::new()),
            tag_allocator: TagAllocator::new(),
        });
        self.servers
            .write()
            .push((self.tag_allocator.allocate(), server_arc.clone()));
        server_arc
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.wait_queue.wake_all();
    }
}
impl Server {
    pub fn request(self: &Self, responder: Responder) {
        self.queue.write().push_back(responder);
        self.wait_queue.wake_one();
    }
    pub fn respond(self: &Self, tag: Tag, response: Message) -> Option<()> {
        let mut waiting_write = self.waiting.write();
        waiting_write
            .pop_if(|tag_responder| tag_responder.0 == tag)?
            .1
            .respond(response)
    }
    pub fn query(self: &Self, tag: Tag) -> Option<QueryResult> {
        self.waiting
            .write()
            .pop_if(|tag_responder| tag_responder.0 == tag)
            .map(|(_, responder)| responder.query())
    }
    pub fn next(self: &Self) -> Option<(Tag, u64)> {
        let mut queue_write = self.queue.write();
        queue_write.retain(|request| match request.query() {
            crate::exchange::QueryResult::Waiting => true,
            crate::exchange::QueryResult::Ready(..) => true,
            crate::exchange::QueryResult::Dropped => false,
        });
        let responder = queue_write.pop_front()?;
        drop(queue_write);
        let length = responder.query().len()?;
        let tag = self.tag_allocator.allocate();
        self.waiting.write().push((tag.clone(), responder));
        Some((tag.clone(), length))
    }
    pub fn block(self: Arc<Self>) -> Option<(Tag, u64)> {
        let self_weak = Arc::downgrade(&self);
        let wait_queue = self.wait_queue.clone();
        drop(self); // allow server to be dropped while waiting
        let responder = loop {
            if let Some(self_arc) = wait_queue.wait_until(|| match self_weak.upgrade() {
                Some(self_arc) => {
                    if self_arc.queue.read().len() > 0 {
                        Some(Some(self_arc))
                    } else {
                        Some(None)
                    }
                }
                None => Some(None),
            }) {
                if let Some(responder) = self_arc.queue.write().pop_front() {
                    break Some(responder);
                } else {
                }
            } else {
                break None;
            }
        }?;
        let new_self = self_weak.upgrade()?;
        let tag = new_self.tag_allocator.allocate();
        let length = responder.query().len()?;
        new_self.waiting.write().push((tag.clone(), responder));
        Some((tag, length))
    }
    pub fn accept(self: &Self, tag: Tag) -> Option<Message> {
        self.waiting
            .read()
            .iter()
            .find(|tag_responder| tag_responder.0 == tag)?
            .1
            .accept()
    }
    pub fn drop(self: &Self, tag: Tag) -> Option<Responder> {
        self.waiting
            .write()
            .pop_if(|tag_responder| tag_responder.0 == tag)
            .map(|(_, responder)| responder)
    }
}
