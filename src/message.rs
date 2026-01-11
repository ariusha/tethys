use alloc::{string::String, vec::Vec};
use crate::{exchange::ValidRequest, state::State};
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
pub enum MessageRequestor {

}