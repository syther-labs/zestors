use crate::*;
use std::any::Any;

/// A type-erased message.
#[derive(Debug)]
pub struct BoxedMessage(Box<dyn Any + Send + 'static>);

impl BoxedMessage {
    /// Create a new `BoxedMessage` from the `Sends<M>`.
    pub fn new<M>(sends: Sends<M>) -> Self
    where
        M: Message + Send + 'static,
        Sends<M>: Send + 'static,
    {
        Self(Box::new(sends))
    }

    /// Downcast the `BoxedMessage` to the `Sends<M>`.
    pub fn downcast<M>(self) -> Result<Sends<M>, Self>
    where
        M: Message + Send + 'static,
    {
        match self.0.downcast() {
            Ok(cast) => Ok(*cast),
            Err(boxed) => Err(Self(boxed)),
        }
    }

    /// Downcast the `BoxedMessage` to the `Sends<M>`, and then return the message
    pub fn downcast_into_msg<M>(self, returns: Returns<M>) -> Result<M, Self>
    where
        M: Message + Send + 'static,
    {
        match self.downcast::<M>() {
            Ok(sends) => Ok(<M::Type as MsgType<M>>::into_msg(sends, returns)),
            Err(boxed) => Err(boxed),
        }
    }
}

#[cfg(test)]
mod test {
    use crate as zestors;
    use crate::*;

    #[test]
    fn boxed_msg() {
        struct Msg1;
        struct Msg2;

        impl Message for Msg1 {
            type Type = ();
        }

        impl Message for Msg2 {
            type Type = ();
        }

        let boxed = BoxedMessage::new::<Msg1>(Msg1);
        assert!(boxed.downcast::<Msg1>().is_ok());

        let boxed = BoxedMessage::new::<Msg1>(Msg1);
        assert!(boxed.downcast::<Msg2>().is_err());
    }
}
