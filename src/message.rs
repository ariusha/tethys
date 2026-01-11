use crate::state::State;

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
    RequestOverwriteForward = 32,
    ResponseOverwriteForward = 33,
    RequestOverwriteBackward = 34,
    ResponseOverwriteBackward = 35,
    RequestTruncateForward = 36,
    ResponseTruncateForward = 37,
    RequestTruncateBackward = 38,
    ResponseTruncateBackward = 39,
    RequestSeekForward = 40,
    ResponseSeekForward = 41,
    RequestSeekBackward = 42,
    ResponseSeekBackward = 43,
    RequestSeekStart = 44,
    ResponseSeekStart = 45,
    RequestSeekEnd = 46,
    ResponseSeekEnd = 47,
    RequestBindBefore = 48,
    ResponseBindBefore = 49,
    RequestBindAfter = 50,
    ResponseBindAfter = 51,
    ResponseFailure = u64::MAX,
}
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
    descriptor: u64,
    count: u64,
}
pub struct MessageRequestListSeekForward {

}
pub struct MessageResponseListSeekForward {}
pub struct MessageRequestListSeekBackward {}
pub struct MessageResponseListSeekBackward {}
pub struct MessageRequestListSeekStart {}
pub struct MessageResponseListSeekStart {}
pub struct MessageRequestListSeekEnd {}
pub struct MessageResponseListSeekEnd {}
pub struct MessageRequestListTell {}
pub struct MessageResponseListTell {}
pub struct MessageRequestMake {}
pub struct MessageResponseMake {}
pub struct MessageRequestRemove {}
pub struct MessageResponseRemove {}
pub struct MessageRequestRename {}
pub struct MessageResponseRename {}
pub struct MessageRequestRead {}
pub struct MessageResponseRead {}
pub struct MessageRequestPeek {}
pub struct MessageResponsePeek {}
pub struct MessageRequestInsert {}
pub struct MessageResponseInsert {}
pub struct MessageRequestOverwriteForward {}
pub struct MessageResponseOverwriteForward {}
pub struct MessageRequestOverwriteBackward {}
pub struct MessageResponseOverwriteBackward {}
pub struct MessageRequestTruncateForward {}
pub struct MessageResponseTruncateForward {}
pub struct MessageRequestTruncateBackward {}
pub struct MessageResponseTruncateBackward {}
pub struct MessageRequestSeekForward {}
pub struct MessageResponseSeekForward {}
pub struct MessageRequestSeekBackward {}
pub struct MessageResponseSeekBackward {}
pub struct MessageRequestSeekStart {}
pub struct MessageResponseSeekStart {}
pub struct MessageRequestSeekEnd {}
pub struct MessageResponseSeekEnd {}
pub struct MessageRequestBindBefore {}
pub struct MessageResponseBindBefore {}
pub struct MessageRequestBindAfter {}
pub struct MessageResponseBindAfter {}
pub enum Message {
}