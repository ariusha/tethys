use crate::{
    memory::{handle_copy, handle_map, handle_switch, handle_unmap},
    process::{Process, Thread},
};
use ostd::sync::RwMutex;
pub enum Syscall {
    // basic
    Abort = 0,
    // memory
    Map = 1,
    Unmap = 2,
    Copy = 3,
    Switch = 4,
    // tethys
    SendRequest = 5,
    SendResponse = 6,
    QueryRequest = 7,
    QueryResponse = 8,
    BlockRequest = 9,
    BlockResponse = 10,
    AcceptRequest = 11,
    AcceptResponse = 12,
    DropRequest = 13,
    DropResponse = 14,
    // debug
    Print = 15,
    // light mutexes
    // NewMutex,
    // DropMutex,
    // AcquireMutex,
    // ReleaseMutex,
}
impl Syscall {
    pub fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::Abort),
            1 => Some(Self::Map),
            2 => Some(Self::Unmap),
            3 => Some(Self::Copy),
            4 => Some(Self::Switch),
            5 => Some(Self::SendRequest),
            6 => Some(Self::SendResponse),
            7 => Some(Self::QueryRequest),
            8 => Some(Self::QueryResponse),
            9 => Some(Self::BlockRequest),
            10 => Some(Self::BlockResponse),
            11 => Some(Self::AcceptRequest),
            12 => Some(Self::AcceptResponse),
            13 => Some(Self::DropRequest),
            14 => Some(Self::DropResponse),
            15 => Some(Self::Print),
            _ => None,
        }
    }
}
const SYSCALL_FAILURE: u64 = 0;
const SYSCALL_SUCCESS: u64 = 1;
pub fn handle_syscall(
    registers: [u64; 5],
    thread: &RwMutex<Thread>,
    process: &Process,
) -> (bool, [Option<u64>; 5]) {
    if let Some(as_syscall) = Syscall::from_u64(registers[0]) {
        match as_syscall {
            Syscall::Abort => (true, [None, None, None, None, None]),
            Syscall::Map => (
                false,
                [
                    Some(if handle_map(&process.memory, registers[1], registers[2]) {
                        SYSCALL_SUCCESS
                    } else {
                        SYSCALL_FAILURE
                    }),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            Syscall::Unmap => (
                false,
                [
                    Some(
                        if handle_unmap(&process.memory, registers[1], registers[2]) {
                            SYSCALL_SUCCESS
                        } else {
                            SYSCALL_FAILURE
                        },
                    ),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            Syscall::Copy => (
                false,
                [
                    Some(
                        if handle_copy(&process.memory, registers[1], registers[2], registers[3]) {
                            SYSCALL_SUCCESS
                        } else {
                            SYSCALL_FAILURE
                        },
                    ),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            Syscall::Switch => (
                false,
                [
                    Some(
                        if handle_switch(&process.memory, registers[1], registers[2], registers[3])
                        {
                            SYSCALL_SUCCESS
                        } else {
                            SYSCALL_FAILURE
                        },
                    ),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
            Syscall::SendRequest => todo!(),
            Syscall::SendResponse => todo!(),
            Syscall::QueryRequest => todo!(),
            Syscall::QueryResponse => todo!(),
            Syscall::BlockRequest => todo!(),
            Syscall::BlockResponse => todo!(),
            Syscall::AcceptRequest => todo!(),
            Syscall::AcceptResponse => todo!(),
            Syscall::DropRequest => todo!(),
            Syscall::DropResponse => todo!(),
            Syscall::Print => todo!(),
        }
    } else {
        (false, [Some(SYSCALL_FAILURE), None, None, None, None])
    }
}
