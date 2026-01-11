#[derive(Debug, Clone)]
pub struct State {
    pub walk: bool,
    pub rename: bool,
    pub make: bool,
    pub remove: bool,
    pub read: bool,
    pub insert: bool,
    pub overwrite: bool,
    pub truncate: bool,
    pub seek_forward: bool,
    pub seek_backward: bool,
    pub seek_start: bool,
    pub seek_end: bool,
    pub tell: bool,
    pub bind: bool,
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
            rename: false,
            make: false,
            remove: false,
            read: false,
            insert: false,
            overwrite: false,
            truncate: false,
            seek_forward: false,
            seek_backward: false,
            seek_start: false,
            seek_end: false,
            tell: false,
            bind: false,
        }
    }
    state_chain!(walk);
    state_chain!(rename);
    state_chain!(make);
    state_chain!(remove);
    state_chain!(read);
    state_chain!(insert);
    state_chain!(overwrite);
    state_chain!(truncate);
    state_chain!(seek_forward);
    state_chain!(seek_backward);
    state_chain!(seek_start);
    state_chain!(seek_end);
    state_chain!(tell);
    state_chain!(bind);
    pub fn intersection(self: &Self, other: &Self) -> State {
        State {
            walk: self.walk & other.walk,
            rename: self.rename & other.rename,
            make: self.make & other.make,
            remove: self.remove & other.remove,
            read: self.read & other.read,
            insert: self.insert & other.insert,
            overwrite: self.overwrite & other.overwrite,
            truncate: self.truncate & other.truncate,
            seek_forward: self.seek_forward & other.seek_forward,
            seek_backward: self.seek_backward & other.seek_backward,
            seek_start: self.seek_start & other.seek_start,
            seek_end: self.seek_end & other.seek_end,
            tell: self.tell & other.tell,
            bind: self.bind & other.bind,
        }
    }
}
