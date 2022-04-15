use core::pin::Pin;

pub trait Message {}

impl Message for () {}

impl<T> Message for Pin<T> {}
