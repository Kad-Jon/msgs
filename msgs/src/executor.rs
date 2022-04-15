use crate::{actor::Actor, context::CoreContext};

pub trait Executor {
    fn run<C, A>()
    where
        A: Actor<Context = C>,
        C: CoreContext<A>;
}
