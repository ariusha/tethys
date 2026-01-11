use crate::{
    message::{
        MessageRequestDrop, MessageRequestList, MessageRequestListPeek,
        MessageRequestListSeekBackward, MessageRequestListSeekEnd, MessageRequestListSeekForward,
        MessageRequestListSeekStart, MessageRequestListTell, MessageRequestMake,
        MessageRequestRemove, MessageRequestState, MessageRequestWalk, MessageResponseDrop,
        MessageResponseList, MessageResponseListPeek, MessageResponseListSeekBackward,
        MessageResponseListSeekEnd, MessageResponseListSeekForward, MessageResponseListSeekStart,
        MessageResponseListTell, MessageResponseMake, MessageResponseRemove, MessageResponseState,
        MessageResponseWalk,
    },
    state::State,
};
use alloc::vec::Vec;
pub fn get_u64(bytes: &[u8], offset: u64) -> Option<u64> {
    let array: [u8; size_of::<u64>()] = bytes
        .get(offset as usize..offset as usize + size_of::<u64>())?
        .try_into()
        .ok()?;
    Some(u64::from_le_bytes(array))
}
pub fn set_u64(bytes: &mut [u8], offset: u64, value: u64) -> Option<()> {
    Some(
        bytes
            .get_mut(offset as usize..offset as usize + size_of::<u64>())?
            .copy_from_slice(&value.to_le_bytes()),
    )
}
pub fn offset<'a>(bytes: &'a [u8]) -> Option<&'a [u8]> {
    bytes.get(get_u64(bytes, 0)? as usize + size_of::<u64>()..).into()
}
pub fn offset_mut<'a>(bytes: &'a mut [u8]) -> Option<&'a mut [u8]> {
    bytes.get_mut(get_u64(bytes, 0)? as usize + size_of::<u64>()..).into()
}
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageTag {
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
impl MessageTag {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::RequestState),
            1 => Some(Self::ResponseState),
            2 => Some(Self::RequestDrop),
            3 => Some(Self::ResponseDrop),
            4 => Some(Self::RequestWalk),
            5 => Some(Self::ResponseWalk),
            6 => Some(Self::RequestList),
            7 => Some(Self::ResponseList),
            8 => Some(Self::RequestListPeek),
            9 => Some(Self::ResponseListPeek),
            10 => Some(Self::RequestListSeekForward),
            11 => Some(Self::ResponseListSeekForward),
            12 => Some(Self::RequestListSeekBackward),
            13 => Some(Self::ResponseListSeekBackward),
            14 => Some(Self::RequestListSeekStart),
            15 => Some(Self::ResponseListSeekStart),
            16 => Some(Self::RequestListSeekEnd),
            17 => Some(Self::ResponseListSeekEnd),
            18 => Some(Self::RequestListTell),
            19 => Some(Self::ResponseListTell),
            20 => Some(Self::RequestMake),
            21 => Some(Self::ResponseMake),
            22 => Some(Self::RequestRemove),
            23 => Some(Self::ResponseRemove),
            24 => Some(Self::RequestRename),
            25 => Some(Self::ResponseRename),
            26 => Some(Self::RequestRead),
            27 => Some(Self::ResponseRead),
            28 => Some(Self::RequestPeek),
            29 => Some(Self::ResponsePeek),
            30 => Some(Self::RequestInsert),
            31 => Some(Self::ResponseInsert),
            32 => Some(Self::RequestOverwrite),
            33 => Some(Self::ResponseOverwrite),
            34 => Some(Self::RequestTruncate),
            35 => Some(Self::ResponseTruncate),
            36 => Some(Self::RequestSeekForward),
            37 => Some(Self::ResponseSeekForward),
            38 => Some(Self::RequestSeekBackward),
            39 => Some(Self::ResponseSeekBackward),
            40 => Some(Self::RequestSeekStart),
            41 => Some(Self::ResponseSeekStart),
            42 => Some(Self::RequestSeekEnd),
            43 => Some(Self::ResponseSeekEnd),
            44 => Some(Self::RequestBindBefore),
            45 => Some(Self::ResponseBindBefore),
            46 => Some(Self::RequestBindAfter),
            47 => Some(Self::ResponseBindAfter),
            u64::MAX => Some(Self::ResponseFailure),
            _ => None,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_u64(get_u64(bytes, 0)?)
    }
}
impl State {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(Self {
            walk: *bytes.get(0)? != 0,
            make: *bytes.get(1)? != 0,
            remove: *bytes.get(2)? != 0,
            rename: *bytes.get(3)? != 0,
            read: *bytes.get(4)? != 0,
            insert: *bytes.get(5)? != 0,
            overwrite_forward: *bytes.get(6)? != 0,
            overwrite_backward: *bytes.get(7)? != 0,
            truncate_forward: *bytes.get(8)? != 0,
            truncate_backward: *bytes.get(9)? != 0,
            seek_forward: *bytes.get(10)? != 0,
            seek_backward: *bytes.get(11)? != 0,
            seek_start: *bytes.get(12)? != 0,
            seek_end: *bytes.get(13)? != 0,
            tell: *bytes.get(14)? != 0,
            bind_before: *bytes.get(15)? != 0,
            bind_after: *bytes.get(16)? != 0,
        })
    }
    fn to_bytes(self: &Self) -> [u8; 17] {
        [
            self.walk as u8,
            self.make as u8,
            self.remove as u8,
            self.rename as u8,
            self.read as u8,
            self.insert as u8,
            self.overwrite_forward as u8,
            self.overwrite_backward as u8,
            self.truncate_forward as u8,
            self.truncate_backward as u8,
            self.seek_forward as u8,
            self.seek_backward as u8,
            self.seek_start as u8,
            self.seek_end as u8,
            self.tell as u8,
            self.bind_before as u8,
            self.bind_after as u8,
        ]
    }
}
impl MessageRequestState {
    pub fn parse(bytes: Vec<u8>) -> Result<MessageRequestState, Vec<u8>> {
        let offset = offset(&bytes).ok_or_else(|| bytes)?;
        Ok(MessageRequestState(bytes))
    }
    pub fn get_descriptor(self: &Self) -> Option<u64> {
        let offset = offset(&bytes)?;
        None
    }
    pub fn set_descriptor(self: &mut Self, descriptor: u64) -> Option<()> {
        let offset = offset(&bytes)?;
        None
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Result<MessageRequest, Vec<u8>> {
        Err(bytes)
    }
}
pub struct Message();
impl Message {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self()
    }
}
pub fn message_collect(bytes: &[&[u8]]) -> Message {
    Message()
}
fn vec_collect(slices: &[&[u8]]) -> Vec<u8> {
    [0u64.to_le_bytes().as_ref()]
        .as_ref()
        .iter()
        .chain(slices.iter())
        .map(|section| section.iter())
        .flatten()
        .map(|byte| byte.clone())
        .collect::<Vec<u8>>()
}
pub fn request_state(descriptor: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestState as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_state(state: &State) -> Message {
    message_collect(&[
        (MessageTag::ResponseState as u64).to_le_bytes().as_ref(),
        state.as_bytes().as_ref(),
    ])
}
pub fn request_drop(descriptor: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestDrop as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_drop() -> Message {
    message_collect(&[(MessageTag::ResponseDrop as u64).to_le_bytes().as_ref()])
}
pub fn request_walk(descriptor: u64, path: &[u8], state: &State) -> Message {
    message_collect(&[
        (MessageTag::RequestWalk as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        (path.len() as u64).to_le_bytes().as_ref(),
        path,
        state.as_bytes().as_ref(),
    ])
}
pub fn response_walk(descriptor: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseWalk as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn request_list(descriptor: u64, count: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestList as u64).to_le_bytes().as_ref(),
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
                (MessageTag::ResponseList as u64)
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
        (MessageTag::RequestListPeek as u64).to_le_bytes().as_ref(),
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
                (MessageTag::ResponseList as u64)
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
        (MessageTag::RequestListSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_forward(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseListSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_backward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestListSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_backward(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseListSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_start(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestListSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_start(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseListSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_seek_end(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestListSeekEnd as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_seek_end(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseListSeekEnd as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_list_tell(descriptor: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestListTell as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn response_list_tell(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseListTell as u64).to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_make(descriptor: u64, state: &State, name: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::RequestMake as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        state.as_bytes().as_ref(),
        name.len().to_le_bytes().as_ref(),
        name,
    ])
}
pub fn response_make(descriptor: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseMake as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
    ])
}
pub fn request_remove(descriptor: u64, name: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::RequestRemove as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        name.len().to_le_bytes().as_ref(),
        name,
    ])
}
pub fn response_remove() -> Message {
    message_collect(&[(MessageTag::ResponseRemove as u64).to_le_bytes().as_ref()])
}
pub fn request_rename(descriptor: u64, from_name: &[u8], to_name: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::RequestRename as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        from_name.len().to_le_bytes().as_ref(),
        from_name,
        to_name.len().to_le_bytes().as_ref(),
        to_name,
    ])
}
pub fn response_rename() -> Message {
    message_collect(&[(MessageTag::ResponseRename as u64).to_le_bytes().as_ref()])
}
pub fn request_read(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestRead as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_read(content: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::ResponseRead as u64).to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn request_peek(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestPeek as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_peek(content: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::ResponsePeek as u64).to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn request_insert(descriptor: u64, content: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::RequestInsert as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn response_insert(length: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseInsert as u64).to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_overwrite(descriptor: u64, content: &[u8]) -> Message {
    message_collect(&[
        (MessageTag::RequestOverwrite as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        content.len().to_le_bytes().as_ref(),
        content,
    ])
}
pub fn response_overwrite(length: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseOverwrite as u64)
            .to_le_bytes()
            .as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_truncate(descriptor: u64, length: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestTruncate as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn response_truncate(length: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseTruncate as u64).to_le_bytes().as_ref(),
        length.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_forward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_forward(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseSeekForward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_backward(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_backward(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseSeekBackward as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_start(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestSeekStart as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_start(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseSeekStart as u64)
            .to_le_bytes()
            .as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn request_seek_end(descriptor: u64, offset: u64) -> Message {
    message_collect(&[
        (MessageTag::RequestSeekEnd as u64).to_le_bytes().as_ref(),
        descriptor.to_le_bytes().as_ref(),
        offset.to_le_bytes().as_ref(),
    ])
}
pub fn response_seek_end(offset: u64) -> Message {
    message_collect(&[
        (MessageTag::ResponseSeekEnd as u64).to_le_bytes().as_ref(),
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
        (MessageTag::RequestBindBefore as u64)
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
    message_collect(&[(MessageTag::ResponseBindBefore as u64)
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
        (MessageTag::RequestBindAfter as u64).to_le_bytes().as_ref(),
        to_descriptor.to_le_bytes().as_ref(),
        from_descriptor.to_le_bytes().as_ref(),
        to_path.len().to_le_bytes().as_ref(),
        to_path,
        mask.as_bytes().as_ref(),
    ])
}
pub fn response_bind_after() -> Message {
    message_collect(&[(MessageTag::ResponseBindAfter as u64)
        .to_le_bytes()
        .as_ref()])
}
