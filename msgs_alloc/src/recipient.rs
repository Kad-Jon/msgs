use alloc::boxed::Box;
use msgs::{address::Sender, message::Message};

/// Implements dynamic dispatch via message passing to any given actor which handles `M`
pub struct Recipient<M: Message> {
    sender: Box<dyn Sender<M>>,
}

impl<M: Message + Send + 'static> Recipient<M> {
    pub fn tell(&mut self, msg: M) {
        self.sender.tell(msg);
    }
}
