use crate::{exchange::ValidRequest, state::State};
use alloc::{string::String, vec::Vec};
const U64_SIZE: usize = size_of::<u64>();
/*
pub struct MessageRequestState {
    descriptor: u64,
}
pub struct MessageResponseState {
    state: State,
}
pub struct MessageRequestDrop {
    descriptor: u64,
}
pub struct MessageResponseDrop {
}
pub struct MessageRequestWalk {
    descriptor: u64,
    path: String,
    mask: State,
}
pub struct MessageResponseWalk {
    descriptor: u64,
}
pub struct MessageRequestList {
    descriptor: u64,
    count: u64,
}
pub struct MessageResponseList {
    names: Vec<String>,
}
pub struct MessageRequestListPeek {
    descriptor: u64,
    count: u64,
}
pub struct MessageResponseListPeek {
    names: Vec<String>,
}
pub struct MessageRequestListSeekForward {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseListSeekForward {
    offset: u64,
}
pub struct MessageRequestListSeekBackward {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseListSeekBackward {
    offset: u64,
}
pub struct MessageRequestListSeekStart {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseListSeekStart {
    offset: u64,
}
pub struct MessageRequestListSeekEnd {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseListSeekEnd {
    offset: u64,
}
pub struct MessageRequestListTell {
    descriptor: u64,
}
pub struct MessageResponseListTell {
    offset: u64,
}
pub struct MessageRequestMake {
    descriptor: u64,
    name: String,
    mask: State,
}
pub struct MessageResponseMake {
    descriptor: u64,
}
pub struct MessageRequestRemove {
    descriptor: u64,
    name: String,
}
pub struct MessageResponseRemove {
}
pub struct MessageRequestRename {
    descriptor: u64,
    from: String,
    to: String,
}
pub struct MessageResponseRename {
}
pub struct MessageRequestRead {
    descriptor: u64,
    length: u64,
}
pub struct MessageResponseRead {
    content: Vec<u8>,
}
pub struct MessageRequestPeek {
    descriptor: u64,
    length: u64,
}
pub struct MessageResponsePeek {
    content: Vec<u8>,
}
pub struct MessageRequestInsert {
    descriptor: u64,
    content: Vec<u8>,
}
pub struct MessageResponseInsert {
    length: u64,
}
pub struct MessageRequestOverwriteForward {
    descriptor: u64,
    content: Vec<u8>,
}
pub struct MessageResponseOverwriteForward {
    length: u64,
}
pub struct MessageRequestOverwriteBackward {
    descriptor: u64,
    content: Vec<u8>,
}
pub struct MessageResponseOverwriteBackward {
    length: u64,
}
pub struct MessageRequestTruncateForward {
    descriptor: u64,
    length: u64,
}
pub struct MessageResponseTruncateForward {
    length: u64,
}
pub struct MessageRequestTruncateBackward {
    descriptor: u64,
    length: u64,
}
pub struct MessageResponseTruncateBackward {
    length: u64,
}
pub struct MessageRequestSeekForward {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseSeekForward {
    offset: u64,
}
pub struct MessageRequestSeekBackward {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseSeekBackward {
    offset: u64,
}
pub struct MessageRequestSeekStart {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseSeekStart {
    offset: u64,
}
pub struct MessageRequestSeekEnd {
    descriptor: u64,
    offset: u64,
}
pub struct MessageResponseSeekEnd {
    offset: u64,
}
pub struct MessageRequestBindBefore {
    to_descriptor: u64,
    to_path: u64,
    from_descriptor: u64,
    mask: State,
}
pub struct MessageResponseBindBefore {
}
pub struct MessageRequestBindAfter {
    to_descriptor: u64,
    to_path: u64,
    from_descriptor: u64,
    mask: State,
}
pub struct MessageResponseBindAfter {
}
pub struct MessageResponseFailure {
}
*/
pub struct MessageRequestState(pub Vec<u8>);
pub struct MessageResponseState(pub Vec<u8>);
pub struct MessageRequestDrop(pub Vec<u8>);
pub struct MessageResponseDrop(pub Vec<u8>);
pub struct MessageRequestWalk(pub Vec<u8>);
pub struct MessageResponseWalk(pub Vec<u8>);
pub struct MessageRequestList(pub Vec<u8>);
pub struct MessageResponseList(pub Vec<u8>);
pub struct MessageRequestListPeek(pub Vec<u8>);
pub struct MessageResponseListPeek(pub Vec<u8>);
pub struct MessageRequestListSeekForward(pub Vec<u8>);
pub struct MessageResponseListSeekForward(pub Vec<u8>);
pub struct MessageRequestListSeekBackward(pub Vec<u8>);
pub struct MessageResponseListSeekBackward(pub Vec<u8>);
pub struct MessageRequestListSeekStart(pub Vec<u8>);
pub struct MessageResponseListSeekStart(pub Vec<u8>);
pub struct MessageRequestListSeekEnd(pub Vec<u8>);
pub struct MessageResponseListSeekEnd(pub Vec<u8>);
pub struct MessageRequestListTell(pub Vec<u8>);
pub struct MessageResponseListTell(pub Vec<u8>);
pub struct MessageRequestMake(pub Vec<u8>);
pub struct MessageResponseMake(pub Vec<u8>);
pub struct MessageRequestRemove(pub Vec<u8>);
pub struct MessageResponseRemove(pub Vec<u8>);
pub struct MessageRequestRename(pub Vec<u8>);
pub struct MessageResponseRename(pub Vec<u8>);
pub struct MessageRequestRead(pub Vec<u8>);
pub struct MessageResponseRead(pub Vec<u8>);
pub struct MessageRequestPeek(pub Vec<u8>);
pub struct MessageResponsePeek(pub Vec<u8>);
pub struct MessageRequestInsert(pub Vec<u8>);
pub struct MessageResponseInsert(pub Vec<u8>);
pub struct MessageRequestOverwriteForward(pub Vec<u8>);
pub struct MessageResponseOverwriteForward(pub Vec<u8>);
pub struct MessageRequestOverwriteBackward(pub Vec<u8>);
pub struct MessageResponseOverwriteBackward(pub Vec<u8>);
pub struct MessageRequestTruncateForward(pub Vec<u8>);
pub struct MessageResponseTruncateForward(pub Vec<u8>);
pub struct MessageRequestTruncateBackward(pub Vec<u8>);
pub struct MessageResponseTruncateBackward(pub Vec<u8>);
pub struct MessageRequestSeekForward(pub Vec<u8>);
pub struct MessageResponseSeekForward(pub Vec<u8>);
pub struct MessageRequestSeekBackward(pub Vec<u8>);
pub struct MessageResponseSeekBackward(pub Vec<u8>);
pub struct MessageRequestSeekStart(pub Vec<u8>);
pub struct MessageResponseSeekStart(pub Vec<u8>);
pub struct MessageRequestSeekEnd(pub Vec<u8>);
pub struct MessageResponseSeekEnd(pub Vec<u8>);
pub struct MessageRequestBindBefore(pub Vec<u8>);
pub struct MessageResponseBindBefore(pub Vec<u8>);
pub struct MessageRequestBindAfter(pub Vec<u8>);
pub struct MessageResponseBindAfter(pub Vec<u8>);
pub struct MessageResponseFailure(pub Vec<u8>);
pub enum Message {
    RequestState(MessageRequestState),
    ResponseState(MessageResponseState),
    RequestDrop(MessageRequestDrop),
    ResponseDrop(MessageResponseDrop),
    RequestWalk(MessageRequestWalk),
    ResponseWalk(MessageResponseWalk),
    RequestList(MessageRequestList),
    ResponseList(MessageResponseList),
    RequestListPeek(MessageRequestListPeek),
    ResponseListPeek(MessageResponseListPeek),
    RequestListSeekForward(MessageRequestListSeekForward),
    ResponseListSeekForward(MessageResponseListSeekForward),
    RequestListSeekBackward(MessageRequestListSeekBackward),
    ResponseListSeekBackward(MessageResponseListSeekBackward),
    RequestListSeekStart(MessageRequestListSeekStart),
    ResponseListSeekStart(MessageResponseListSeekStart),
    RequestListSeekEnd(MessageRequestListSeekEnd),
    ResponseListSeekEnd(MessageResponseListSeekEnd),
    RequestListTell(MessageRequestListTell),
    ResponseListTell(MessageResponseListTell),
    RequestMake(MessageRequestMake),
    ResponseMake(MessageResponseMake),
    RequestRemove(MessageRequestRemove),
    ResponseRemove(MessageResponseRemove),
    RequestRename(MessageRequestRename),
    ResponseRename(MessageResponseRename),
    RequestRead(MessageRequestRead),
    ResponseRead(MessageResponseRead),
    RequestPeek(MessageRequestPeek),
    ResponsePeek(MessageResponsePeek),
    RequestInsert(MessageRequestInsert),
    ResponseInsert(MessageResponseInsert),
    RequestOverwriteForward(MessageRequestOverwriteForward),
    ResponseOverwriteForward(MessageResponseOverwriteForward),
    RequestOverwriteBackward(MessageRequestOverwriteBackward),
    ResponseOverwriteBackward(MessageResponseOverwriteBackward),
    RequestTruncateForward(MessageRequestTruncateForward),
    ResponseTruncateForward(MessageResponseTruncateForward),
    RequestTruncateBackward(MessageRequestTruncateBackward),
    ResponseTruncateBackward(MessageResponseTruncateBackward),
    RequestSeekForward(MessageRequestSeekForward),
    ResponseSeekForward(MessageResponseSeekForward),
    RequestSeekBackward(MessageRequestSeekBackward),
    ResponseSeekBackward(MessageResponseSeekBackward),
    RequestSeekStart(MessageRequestSeekStart),
    ResponseSeekStart(MessageResponseSeekStart),
    RequestSeekEnd(MessageRequestSeekEnd),
    ResponseSeekEnd(MessageResponseSeekEnd),
    RequestBindBefore(MessageRequestBindBefore),
    ResponseBindBefore(MessageResponseBindBefore),
    RequestBindAfter(MessageRequestBindAfter),
    ResponseBindAfter(MessageResponseBindAfter),
    ResponseFailure(MessageResponseFailure),
}
pub enum MessageRequestor {}
pub fn get_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    let array: [u8; U64_SIZE] = bytes.get(offset..offset + U64_SIZE)?.try_into().ok()?;
    Some(u64::from_le_bytes(array))
}
pub fn set_u64(bytes: &mut [u8], offset: usize, value: u64) -> Option<()> {
    Some(
        bytes
            .get_mut(offset..offset + U64_SIZE)?
            .copy_from_slice(&value.to_le_bytes()),
    )
}
pub fn offset<'a>(bytes: &'a [u8]) -> Option<&'a [u8]> {
    bytes.get(get_u64(bytes, 0)? as usize + U64_SIZE..).into()
}
pub fn offset_mut<'a>(bytes: &'a mut [u8]) -> Option<&'a mut [u8]> {
    bytes
        .get_mut(get_u64(bytes, 0)? as usize + U64_SIZE..)
        .into()
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
macro_rules! message_default {
    ($kind:ident) => {
        impl $kind {
            pub fn take(self: Self) -> Vec<u8> {
                self.0
            }
            pub fn get_tag(self: &Self) -> Option<MessageTag> {
                let content = offset(self.0.as_slice())?;
                Some(MessageTag::from_u64(get_u64(content, 0 * U64_SIZE)?)?)
            }
            pub fn set_tag(self: &mut Self, tag: u64) -> Option<()> {
                let content = offset_mut(self.0.as_mut_slice())?;
                Some(set_u64(content, 0 * U64_SIZE, tag)?)
            }
        }
    };
}
macro_rules! request_default {
    ($kind:ident) => {
        message_default!($kind)
        impl $kind {
            pub fn get_descriptor(self: &Self) -> Option<u64> {
                let content = offset(self.0.as_slice())?;
                Some(get_u64(content, 1 * U64_SIZE)?)
            }
            pub fn set_descriptor(self: &mut Self, descriptor: u64) -> Option<()> {
                let content = offset_mut(self.0.as_mut_slice())?;
                Some(set_u64(content, 1 * U64_SIZE, descriptor)?)
            }
        }
    }
}
request_default!(RequestState);
message_default!(ResponseState);
request_default!(RequestDrop);
message_default!(ResponseDrop);
request_default!(RequestWalk);
message_default!(ResponseWalk);
request_default!(RequestList);
message_default!(ResponseList);
request_default!(RequestListPeek);
message_default!(ResponseListPeek);
request_default!(RequestListSeekForward);
message_default!(ResponseListSeekForward);
request_default!(RequestListSeekBackward);
message_default!(ResponseListSeekBackward);
request_default!(RequestListSeekStart);
message_default!(ResponseListSeekStart);
request_default!(RequestListSeekEnd);
message_default!(ResponseListSeekEnd);
request_default!(RequestListTell);
message_default!(ResponseListTell);
request_default!(RequestMake);
message_default!(ResponseMake);
request_default!(RequestRemove);
message_default!(ResponseRemove);
request_default!(RequestRename);
message_default!(ResponseRename);
request_default!(RequestRead);
message_default!(ResponseRead);
request_default!(RequestPeek);
message_default!(ResponsePeek);
request_default!(RequestInsert);
message_default!(ResponseInsert);
request_default!(RequestOverwrite);
message_default!(ResponseOverwrite);
request_default!(RequestTruncate);
message_default!(ResponseTruncate);
request_default!(RequestSeekForward);
message_default!(ResponseSeekForward);
request_default!(RequestSeekBackward);
message_default!(ResponseSeekBackward);
request_default!(RequestSeekStart);
message_default!(ResponseSeekStart);
request_default!(RequestSeekEnd);
message_default!(ResponseSeekEnd);
request_default!(RequestBindBefore);
message_default!(ResponseBindBefore);
request_default!(RequestBindAfter);
message_default!(ResponseBindAfter);
message_default!(ResponseFailure);
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageRequest {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageRequest> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageRequest {
            return None;
        }
        let descriptor = get_u64(content, 1 * U64_SIZE)?;
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self, ) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}
impl MessageResponse {
    pub fn parse(bytes: Vec<u8>) -> Option<MessageResponse> {
        let content = offset(&bytes)?;
        if content.len() != 0 * U64_SIZE {
            return None;
        }
        let tag = get_u64(content, 0 * U64_SIZE)?;
        if tag != MessageTag::MessageResponse {
            return None;
        }
        Some(MessageRequest(bytes))
    }
    pub fn get_(self: &Self) -> Option {
        let content = offset(self.0.as_slice())?;
        Some()
    }
    pub fn set_(self: &mut Self) -> Option<()> {
        let content = offset_mut(self.0.as_mut_slice())?;
        Some()
    }
}