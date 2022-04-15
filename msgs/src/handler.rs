use super::{actor::Actor, message::Message};

pub trait Handler<M>
where
    M: Message,
    Self: Actor,
{
    fn handle(&mut self, message: M, ctx: &mut Self::Context);
}
