use super::{actor::Actor, handler::Handler, message::Message};

pub trait CoreAddress<A>
where
    A: Actor,
{
    fn tell<M>(&mut self, msg: M)
    where
        M: Message + Send + 'static,
        A: Handler<M>;
}

pub trait Sender<M> {
    fn tell(&mut self, msg: M)
    where
        M: Message + Send + 'static;
}
