use super::context::CoreContext;

pub trait Actor: Sized + Unpin + 'static {
    type Context: CoreContext<Self>;
}
