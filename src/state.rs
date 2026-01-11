#[derive(Debug, Clone)]
pub struct State {
    pub walk: bool,
    pub make: bool,
    pub remove: bool,
    pub rename: bool,
    pub read: bool,
    pub insert: bool,
    pub overwrite_forward: bool,
    pub overwrite_backward: bool,
    pub truncate_forward: bool,
    pub truncate_backward: bool,
    pub seek_forward: bool,
    pub seek_backward: bool,
    pub seek_start: bool,
    pub seek_end: bool,
    pub tell: bool,
    pub bind_before: bool,
    pub bind_after: bool,
}
macro_rules! state_chain {
    ($field:ident) => {
        pub fn $field(mut self: Self, value: bool) -> Self {
            self.$field = value;
            self
        }
    };
}
impl State {
    pub fn new() -> State {
        State {
            walk: false,
            make: false,
            remove: false,
            rename: false,
            read: false,
            insert: false,
            overwrite_forward: false,
            overwrite_backward: false,
            truncate_forward: false,
            truncate_backward: false,
            seek_forward: false,
            seek_backward: false,
            seek_start: false,
            seek_end: false,
            tell: false,
            bind_before: false,
            bind_after: false,
        }
    }
    state_chain!(walk);
    state_chain!(make);
    state_chain!(remove);
    state_chain!(rename);
    state_chain!(read);
    state_chain!(insert);
    state_chain!(overwrite_forward);
    state_chain!(overwrite_backward);
    state_chain!(truncate_forward);
    state_chain!(truncate_backward);
    state_chain!(seek_forward);
    state_chain!(seek_backward);
    state_chain!(seek_start);
    state_chain!(seek_end);
    state_chain!(tell);
    state_chain!(bind_before);
    state_chain!(bind_after);
    pub fn intersection(self: &Self, other: &Self) -> State {
        State {
            walk: self.walk & other.walk,
            rename: self.rename & other.rename,
            make: self.make & other.make,
            remove: self.remove & other.remove,
            read: self.read & other.read,
            insert: self.insert & other.insert,
            overwrite_forward: self.overwrite_forward & other.overwrite_forward,
            overwrite_backward: self.overwrite_backward & other.overwrite_backward,
            truncate_forward: self.truncate_forward & other.truncate_forward,
            truncate_backward: self.truncate_backward & other.truncate_backward,
            seek_forward: self.seek_forward & other.seek_forward,
            seek_backward: self.seek_backward & other.seek_backward,
            seek_start: self.seek_start & other.seek_start,
            seek_end: self.seek_end & other.seek_end,
            tell: self.tell & other.tell,
            bind_before: self.bind_before & other.bind_before,
            bind_after: self.bind_after & other.bind_after,
        }
    }
}
