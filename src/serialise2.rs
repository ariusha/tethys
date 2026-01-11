use crate::{memory::Message, state::State};
use alloc::vec::Vec;
#[repr(u64)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MessageKind {
    RequestState = 0,
    ResponseState = 1,
    RequestDrop = 2,
    ResponseDrop = 3,
    RequestWalk = 4,
    ResponseWalk = 5,
    RequestList = 6,
    ResponseList = 7,
    RequestListPeek = 8,
    ResponseListPeek = 9,
    RequestListSeekForward = 10,
    ResponseListSeekForward = 11,
    RequestListSeekBackward = 12,
    ResponseListSeekBackward = 13,
    RequestListSeekStart = 14,
    ResponseListSeekStart = 15,
    RequestListSeekEnd = 16,
    ResponseListSeekEnd = 17,
    RequestListTell = 18,
    ResponseListTell = 19,
    RequestMake = 20,
    ResponseMake = 21,
    RequestRemove = 22,
    ResponseRemove = 23,
    RequestRename = 24,
    ResponseRename = 25,
    RequestRead = 26,
    ResponseRead = 27,
    RequestPeek = 28,
    ResponsePeek = 29,
    RequestInsert = 30,
    ResponseInsert = 31,
    RequestOverwrite = 32,
    ResponseOverwrite = 33,
    RequestTruncate = 34,
    ResponseTruncate = 35,
    RequestSeekForward = 36,
    ResponseSeekForward = 37,
    RequestSeekBackward = 38,
    ResponseSeekBackward = 39,
    RequestSeekStart = 40,
    ResponseSeekStart = 41,
    RequestSeekEnd = 42,
    ResponseSeekEnd = 43,
    RequestBindBefore = 44,
    ResponseBindBefore = 45,
    RequestBindAfter = 46,
    ResponseBindAfter = 47,
    ResponseFailure = u64::MAX,
}
impl State {
    pub fn as_bytes(self: &Self) -> [u8; 14] {
        [
            self.walk as u8,
            self.rename as u8,
            self.make as u8,
            self.remove as u8,
            self.read as u8,
            self.insert as u8,
            self.overwrite as u8,
            self.truncate as u8,
            self.seek_forward as u8,
            self.seek_backward as u8,
            self.seek_start as u8,
            self.seek_end as u8,
            self.tell as u8,
            self.bind as u8,
        ]
    }
    pub fn from_bytes(bytes: &[u8; 14]) -> State {
        State {
            walk: bytes[0] != 0,
            rename: bytes[1] != 0,
            make: bytes[2] != 0,
            remove: bytes[3] != 0,
            read: bytes[4] != 0,
            insert: bytes[5] != 0,
            overwrite: bytes[6] != 0,
            truncate: bytes[7] != 0,
            seek_forward: bytes[8] != 0,
            seek_backward: bytes[9] != 0,
            seek_start: bytes[10] != 0,
            seek_end: bytes[11] != 0,
            tell: bytes[12] != 0,
            bind: bytes[13] != 0,
        }
    }
}
impl MessageKind {
    pub fn from_u64(value: u64) -> Option<MessageKind> {
        match value {
            0 => Some(MessageKind::RequestState),
            1 => Some(MessageKind::ResponseState),
            2 => Some(MessageKind::RequestDrop),
            3 => Some(MessageKind::ResponseDrop),
            4 => Some(MessageKind::RequestWalk),
            5 => Some(MessageKind::ResponseWalk),
            6 => Some(MessageKind::RequestList),
            7 => Some(MessageKind::ResponseList),
            8 => Some(MessageKind::RequestListPeek),
            9 => Some(MessageKind::ResponseListPeek),
            10 => Some(MessageKind::RequestListSeekForward),
            11 => Some(MessageKind::ResponseListSeekForward),
            12 => Some(MessageKind::RequestListSeekBackward),
            13 => Some(MessageKind::ResponseListSeekBackward),
            14 => Some(MessageKind::RequestListSeekStart),
            15 => Some(MessageKind::ResponseListSeekStart),
            16 => Some(MessageKind::RequestListSeekEnd),
            17 => Some(MessageKind::ResponseListSeekEnd),
            18 => Some(MessageKind::RequestListTell),
            19 => Some(MessageKind::ResponseListTell),
            20 => Some(MessageKind::RequestMake),
            21 => Some(MessageKind::ResponseMake),
            22 => Some(MessageKind::RequestRemove),
            23 => Some(MessageKind::ResponseRemove),
            24 => Some(MessageKind::RequestRename),
            25 => Some(MessageKind::ResponseRename),
            26 => Some(MessageKind::RequestRead),
            27 => Some(MessageKind::ResponseRead),
            28 => Some(MessageKind::RequestPeek),
            29 => Some(MessageKind::ResponsePeek),
            30 => Some(MessageKind::RequestInsert),
            31 => Some(MessageKind::ResponseInsert),
            32 => Some(MessageKind::RequestOverwrite),
            33 => Some(MessageKind::ResponseOverwrite),
            34 => Some(MessageKind::RequestTruncate),
            35 => Some(MessageKind::ResponseTruncate),
            36 => Some(MessageKind::RequestSeekForward),
            37 => Some(MessageKind::ResponseSeekForward),
            38 => Some(MessageKind::RequestSeekBackward),
            39 => Some(MessageKind::ResponseSeekBackward),
            40 => Some(MessageKind::RequestSeekStart),
            41 => Some(MessageKind::ResponseSeekStart),
            42 => Some(MessageKind::RequestSeekEnd),
            43 => Some(MessageKind::ResponseSeekEnd),
            44 => Some(MessageKind::RequestBindBefore),
            45 => Some(MessageKind::ResponseBindBefore),
            46 => Some(MessageKind::RequestBindAfter),
            47 => Some(MessageKind::ResponseBindAfter),
            u64::MAX => Some(MessageKind::ResponseFailure),
            _ => None,
        }
    }
}
fn message_collect(slices: &[&[u8]]) -> Message {
    Message::new(
        [0u64.to_le_bytes().as_ref()]
            .as_ref()
            .iter()
            .chain(slices.iter())
            .map(|section| section.iter())
            .flatten()
            .map(|byte| byte.clone())
            .collect::<Vec<u8>>(),
    )
}
pub fn request_state(descriptor: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestState as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_state(state: &State) -> Message {
    message_collect(&[
        (MessageKind::ResponseState as u64).to_le_bytes().as_ref(),
        state.as_bytes().as_ref(),
    ])
}
pub fn request_drop(descriptor: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestDrop as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_drop() -> Message {
    message_collect(&[(MessageKind::ResponseDrop as u64).to_le_bytes().as_ref()])
}
pub fn request_walk(descriptor: u64, path: &[u8], state: &State) -> Message {
    message_collect(&[
        (MessageKind::RequestWalk as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        (path.len() as u64).to_le_bytes().as_ref(),
        path,
        state.as_bytes().as_ref(),
    ])
}
pub fn response_walk(descriptor: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseWalk as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn request_list(descriptor: u64, count: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestList as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        count.to_le_bytes().as_ref(),
    ])
}
pub fn response_list(names: &[&[u8]]) -> Message {
    let mut offset = 0;
    let offsets: Vec<u64> = names
        .iter()
        .map(|name| {
            let current = offset as u64;
            offset += name.len();
            current
        })
        .collect();
    Message::new(
        0u64.to_le_bytes()
            .iter()
            .copied()
            .chain(
                (MessageKind::ResponseList as u64)
                    .to_le_bytes()
                    .iter()
                    .copied(),
            )
            .chain((names.len() as u64).to_le_bytes().iter().copied())
            .chain(
                offsets
                    .iter()
                    .flat_map(|&off| off.to_le_bytes().into_iter()),
            )
            .chain(names.iter().flat_map(|name| name.iter().copied()))
            .collect(),
    )
}
pub fn request_list_peek(descriptor: u64, count: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListPeek as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        count.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_peek(names: &[&[u8]]) -> Message {
    let mut offset = 0;
    let offsets: Vec<u64> = names
        .iter()
        .map(|name| {
            let current = offset as u64;
            offset += name.len();
            current
        })
        .collect();
    Message::new(
        0u64.to_le_bytes()
            .iter()
            .copied()
            .chain(
                (MessageKind::ResponseList as u64)
                    .to_le_bytes()
                    .iter()
                    .copied(),
            )
            .chain((names.len() as u64).to_le_bytes().iter().copied())
            .chain(
                offsets
                    .iter()
                    .flat_map(|&off| off.to_le_bytes().into_iter()),
            )
            .chain(names.iter().flat_map(|name| name.iter().copied()))
            .collect(),
    )
}
pub fn request_list_seek_forward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_forward(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseListSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_backward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_backward(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseListSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_start(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_start(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseListSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_end(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListSeekEnd as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_end(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseListSeekEnd as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_tell(descriptor: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestListTell as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_tell(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseListTell as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_make(descriptor: u64, state: &State, name: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::RequestMake as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        state.as_bytes().as_ref(),
        name.len().to_le_bytes().as_ref(),
        name,
    ])
}
pub fn response_make(descriptor: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseMake as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn request_remove(descriptor: u64, name: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::RequestRemove as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        name.len().to_le_bytes().as_ref(),
        name,
    ])
}
pub fn response_remove() -> Message {
    message_collect(&[(MessageKind::ResponseRemove as u64).to_le_bytes().as_ref()])
}
pub fn request_rename(descriptor: u64, from_name: &[u8], to_name: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::RequestRename as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        from_name.len().to_le_bytes().as_ref(),
        from_name,
        to_name.len().to_le_bytes().as_ref(),
        to_name,
    ])
}
pub fn response_rename() -> Message {
    message_collect(&[(MessageKind::ResponseRename as u64).to_le_bytes().as_ref()])
}
pub fn request_read(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestRead as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_read(content: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::ResponseRead as u64).to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn request_peek(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestPeek as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_peek(content: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::ResponsePeek as u64).to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn request_insert(descriptor: u64, content: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::RequestInsert as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn response_insert(length: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseInsert as u64).to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_overwrite(descriptor: u64, content: &[u8]) -> Message {
    message_collect(&[
        (MessageKind::RequestOverwrite as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn response_overwrite(length: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseOverwrite as u64)
            .to_le_bytes()
            .as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_truncate(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestTruncate as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_truncate(length: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseTruncate as u64)
            .to_le_bytes()
            .as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_forward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_forward(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_backward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_backward(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_start(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_start(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_end(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageKind::RequestSeekEnd as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_end(offset: u64) -> Message {
    message_collect(&[
        (MessageKind::ResponseSeekEnd as u64).to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_bind_before(
    to_descriptor: u64,
    from_descriptor: u64,
    to_path: &[u8],
    mask: &State,
) -> Message {
    message_collect(&[
        (MessageKind::RequestBindBefore as u64)
            .to_le_bytes()
            .as_ref(),
        to_descriptor.to_le_bytes().as_ref(),
        from_descriptor.to_le_bytes().as_ref(),
        to_path.len().to_le_bytes().as_ref(),
        to_path,
        mask.as_bytes().as_ref(),
    ])
}
pub fn response_bind_before() -> Message {
    message_collect(&[(MessageKind::ResponseBindBefore as u64)
        .to_le_bytes()
        .as_ref()])
}
pub fn request_bind_after(
    to_descriptor: u64,
    from_descriptor: u64,
    to_path: &[u8],
    mask: &State,
) -> Message {
    message_collect(&[
        (MessageKind::RequestBindAfter as u64)
            .to_le_bytes()
            .as_ref(),
        to_descriptor.to_le_bytes().as_ref(),
        from_descriptor.to_le_bytes().as_ref(),
        to_path.len().to_le_bytes().as_ref(),
        to_path,
        mask.as_bytes().as_ref(),
    ])
}
pub fn response_bind_after() -> Message {
    message_collect(&[(MessageKind::ResponseBindAfter as u64)
        .to_le_bytes()
        .as_ref()])
}
