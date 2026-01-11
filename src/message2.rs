use alloc::boxed::Box;
use crate::{
    exchange::exchange, memory::Message, serialise::MessageKind, state::State,
    tethys::ProcessTethysData,
};
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
pub fn get_u64(bytes: &[u8], index: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        bytes
            .get(index * size_of::<u64>()..(index + 1) * size_of::<u64>())?
            .try_into()
            .ok()?,
    ))
}
pub fn handle_send(process_tethys: &ProcessTethysData, message: Message) -> Option<()> {
    let request_offset = get_u64(message.peek(), 0)?;
    let request_content = message.peek().get(request_offset as usize..)?;
    match MessageKind::from_u64(get_u64(request_content, 0)?)? {
        MessageKind::RequestState => {
            let descriptor_tag = get_u64(request_content, 1)?;
            let descriptor_read = process_tethys
                .descriptors
                .read()
                .iter()
                .find(|descriptor| descriptor.read().tag.0 == descriptor_tag)?
                .read();
            let descriptor_mask = descriptor_read.mask;
            let (requestor, responder) = exchange(
                message,
                Box::new(|requestor| requestor.accept()),
                Box::new(|responder, mut response| {
                    let response_offset = get_u64(response.peek(), 0)?;
                    let response_content = response
                        .peek_mut()
                        .get_mut(response_offset as usize..)?;
                    let state = (&*response_content)
                    .get(size_of::<u64>()..)
                    .and_then(
                        |state_bytes| state_bytes
                        .try_into()
                        .ok()
                        .map(|state_exact| State::from_bytes(state_exact))
                    )?;
                    let mask = descriptor_mask;
                    let new_bytes = state.intersection(&mask).as_bytes();
                    response_content.get(size_of::<u64>()..size_of::<u64>() + new_bytes.len())?;
                    responder.respond(response)
                }),
            );
            Some(descriptor_read.server.upgrade()?.request(responder))
        }
        MessageKind::RequestDrop => {
            let descriptor_tag = get_u64(request_content, 1)?;
            process_tethys
                .descriptors
                .write()
                .pop_if(|descriptor| descriptor.read().tag.0 == descriptor_tag)
                .map(|_| ())
        }
        MessageKind::RequestWalk => todo!(),
        MessageKind::RequestList => todo!(),
        MessageKind::RequestListPeek => todo!(),
        MessageKind::RequestListSeekForward => todo!(),
        MessageKind::RequestListSeekBackward => todo!(),
        MessageKind::RequestListSeekStart => todo!(),
        MessageKind::RequestListSeekEnd => todo!(),
        MessageKind::RequestListTell => todo!(),
        MessageKind::RequestMake => todo!(),
        MessageKind::RequestRemove => todo!(),
        MessageKind::RequestRename => todo!(),
        MessageKind::RequestRead => todo!(),
        MessageKind::RequestPeek => todo!(),
        MessageKind::RequestInsert => todo!(),
        MessageKind::RequestOverwrite => todo!(),
        MessageKind::RequestTruncate => todo!(),
        MessageKind::RequestSeekForward => todo!(),
        MessageKind::RequestSeekBackward => todo!(),
        MessageKind::RequestSeekStart => todo!(),
        MessageKind::RequestSeekEnd => todo!(),
        MessageKind::RequestBindBefore => todo!(),
        MessageKind::RequestBindAfter => todo!(),
        _ => None,
    }
}
