use crate::{address::CoreAddress, envelope::Envelope};

use super::actor::Actor;

pub trait CoreContext<A>
where
    A: Actor<Context = Self>,
    Self: Sized,
{
    fn stop(&mut self);

    fn start<Addr>(&mut self, act: A) -> Addr;

    fn run<B>(mb: B) -> ActorExecutable<A, Self, B>
    where
        B: Mailbox<A>;
}

pub trait AddressableBy<Addr, A>
where
    A: Actor<Context = Self>,
    Addr: CoreAddress<A>,
    Self: CoreContext<A>,
{
}

pub trait Mailbox<A: Actor> {
    type Envelope: Envelope<A>;

    fn next(&mut self) -> Option<Self::Envelope>;

    fn capacity(&self) -> usize;

    fn set_capacity(&mut self, size: usize);
}

pub struct ActorExecutable<A, C, B>
where
    A: Actor<Context = C>,
    C: CoreContext<A>,
    B: Mailbox<A>,
{
    act: A,
    ctx: C,
    mb: B,
}

impl<A, C, B> ActorExecutable<A, C, B>
where
    A: Actor<Context = C>,
    C: CoreContext<A>,
    B: Mailbox<A>,
{
}

pub enum ActorExecutionState {
    Started,
    Running,
    Stopping,
    Stopped,
}
