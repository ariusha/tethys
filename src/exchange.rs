use alloc::{
    boxed::Box,
    sync::{Arc, UniqueArc, Weak},
};
use core::sync::atomic::{AtomicBool, Ordering};
use ostd::sync::{RwMutex, WaitQueue};
pub enum RequestorQueryError {
    Waiting,
    Dropped,
}
pub enum RequestorBlockError {
    Dropped,
}
pub enum RequestorAcceptError {
    Dropped,
}
pub enum RequestorRespondError {
    Dropped,
}
pub enum ResponderQueryError {
    Dropped,
    Waiting,
}
pub enum ResponderBlockError {
    Dropped,
}
pub enum ResponderAcceptError {
    Dropped,
}
pub enum ResponderRespondError {
    Dropped,
}
pub struct RequestorInner<Request: ValidRequest<Response>, Response: ValidResponse<Request>> {
    responder: Weak<ResponderInner<Request, Response>>,
    response: RwMutex<Option<Response>>,
    responder_dropped: AtomicBool,
}
pub struct ResponderInner<Request: ValidRequest<Response>, Response: ValidResponse<Request>> {
    requestor: Weak<RequestorInner<Request, Response>>,
    request: RwMutex<Option<Request>>,
    wait_queue: Arc<WaitQueue>,
    requestor_forgotten: AtomicBool,
}
pub trait ValidResponse<Request> {
    fn len(self: &Self) -> u64;
}
pub trait ValidRequest<Response> {
    fn len(self: &Self) -> u64;
}
pub struct RequestorTyped<
    Request: ValidRequest<Response>,
    Response: ValidResponse<Request>,
    RequestorExtra: FnOnce(Response) -> Result<Response, RequestorAcceptError>,
>(Arc<RequestorInner<Request, Response>>, Box<RequestorExtra>);
pub struct ResponderTyped<
    Request: ValidRequest<Response>,
    Response: ValidResponse<Request>,
    ResponderExtra: FnOnce(Response) -> Result<Response, ResponderRespondError>,
>(Arc<ResponderInner<Request, Response>>, Box<ResponderExtra>);
pub fn exchange<
    Request: ValidRequest<Response>,
    Response: ValidResponse<Request>,
    RequestorExtra: FnOnce(Response) -> Result<Response, RequestorAcceptError>,
    ResponderExtra: FnOnce(Response) -> Result<Response, ResponderRespondError>,
>(
    request: Request,
    requestor_extra: RequestorExtra,
    responder_extra: ResponderExtra,
) -> (
    RequestorTyped<Request, Response, RequestorExtra>,
    ResponderTyped<Request, Response, ResponderExtra>,
) {
    let mut requestor = UniqueArc::new(RequestorInner {
        responder: Weak::new(),
        response: RwMutex::new(None),
        responder_dropped: AtomicBool::new(false),
    });
    let responder = Arc::new(ResponderInner {
        requestor: UniqueArc::downgrade(&requestor),
        request: RwMutex::new(Some(request)),
        wait_queue: Arc::new(WaitQueue::new()),
        requestor_forgotten: AtomicBool::new(false),
    });
    requestor.responder = Arc::downgrade(&responder);
    (
        RequestorTyped(UniqueArc::into_arc(requestor), Box::new(requestor_extra)),
        ResponderTyped(responder, Box::new(responder_extra)),
    )
}
impl<Request: ValidRequest<Response>, Response: ValidResponse<Request>> Drop
    for ResponderInner<Request, Response>
{
    fn drop(&mut self) {
        self.requestor
            .upgrade()
            .map(|requestor| requestor.responder_dropped.store(true, Ordering::Relaxed));
        self.wait_queue.wake_all();
    }
}
impl<
    Request: ValidRequest<Response>,
    Response: ValidResponse<Request>,
    RequestorExtra: FnOnce(Response) -> Result<Response, RequestorAcceptError>,
> RequestorTyped<Request, Response, RequestorExtra>
{
    pub fn forget(self: Self) {
        self.0
            .responder
            .upgrade()
            .map(|responder| responder.requestor_forgotten.store(true, Ordering::Relaxed));
    }
    pub fn query(self: &Self) -> Result<u64, RequestorQueryError> {
        match &*self.0.response.read() {
            Some(response) => Ok(response.len() as u64),
            None => {
                if self.0.responder_dropped.load(Ordering::Relaxed) {
                    Err(RequestorQueryError::Dropped)
                } else {
                    match self.0.responder.upgrade() {
                        Some(..) => Err(RequestorQueryError::Waiting),
                        None => Err(RequestorQueryError::Dropped),
                    }
                }
            }
        }
    }
    pub fn block(self: &Self) -> Result<u64, RequestorBlockError> {
        let wait_queue = self
            .0
            .responder
            .upgrade()
            .ok_or_else(|| RequestorBlockError::Dropped)?
            .wait_queue
            .clone();
        wait_queue.wait_until(|| {
            if self.0.responder_dropped.load(Ordering::Relaxed) {
                Some(Err(RequestorBlockError::Dropped))
            } else {
                match self.query() {
                    Ok(length) => Some(Ok(length)),
                    Err(error) => match error {
                        RequestorQueryError::Waiting => None,
                        RequestorQueryError::Dropped => Some(Err(RequestorBlockError::Dropped)),
                    },
                }
            }
        })
    }
    pub fn accept(self: Self) -> Result<Response, RequestorAcceptError> {
        match self.block() {
            Ok(..) => (),
            Err(error) => match error {
                RequestorBlockError::Dropped => return Err(RequestorAcceptError::Dropped),
            },
        };
        match self.0.response.write().take() {
            Some(response) => {
                let Self(_, extra) = self;
                extra(response)
            }
            None => Err(RequestorAcceptError::Dropped),
        }
    }
}
impl<
    Request: ValidRequest<Response>,
    Response: ValidResponse<Request>,
    ResponderExtra: FnOnce(Response) -> Result<Response, ResponderRespondError>,
> ResponderTyped<Request, Response, ResponderExtra>
{
    pub fn query(self: &Self) -> Result<u64, ResponderQueryError> {
        let inner_length_result = match &*self.0.request.read() {
            Some(request) => Ok(request.len() as u64),
            None => Err(ResponderQueryError::Waiting),
        };
        match self.0.requestor.upgrade() {
            Some(..) => inner_length_result,
            None => {
                if self.0.requestor_forgotten.load(Ordering::Relaxed) {
                    inner_length_result
                } else {
                    Err(ResponderQueryError::Dropped)
                }
            }
        }
    }
    pub fn accept(self: &Self) -> Result<Request, ResponderAcceptError> {
        match self.0.request.write().take() {
            Some(request) => Ok(request),
            None => Err(ResponderAcceptError::Dropped),
        }
    }
    pub fn respond(self: Self, response: Response) -> Result<(), ResponderRespondError> {
        match self.0.requestor.upgrade() {
            Some(requestor) => match self.1(response) {
                Ok(injected) => Ok(*requestor.response.write() = Some(injected)),
                Err(error) => Err(error),
            },
            None => Err(ResponderRespondError::Dropped),
        }
    }
}
