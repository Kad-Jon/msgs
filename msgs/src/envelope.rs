use crate::{actor::Actor, handler::Handler, message::Message};

pub trait Envelope<A>
where
    Self: Sized,
    A: Actor,
{
    fn pack<M>(msg: M) -> Self
    where
        M: Message,
        A: Handler<M>;

    fn unpack<M>(self) -> Option<M>
    where
        M: Message,
        A: Handler<M>;

    fn handle(self, act: &mut A, ctx: &mut A::Context);
}
