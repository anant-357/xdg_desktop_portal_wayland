use std::collections::HashMap;

use serde::Serialize;
use zbus::{interface, zvariant};
pub struct Request {
    pub handle_path: zvariant::OwnedObjectPath,
}

#[interface(name = "org.freedesktop.impl.portal.Request")]
impl Request {
    async fn close(
        &self,
        #[zbus(object_server)] server: &zbus::ObjectServer,
    ) -> zbus::fdo::Result<()> {
        server
            .remove::<Self, &zvariant::OwnedObjectPath>(&self.handle_path)
            .await?;
        Ok(())
    }
}

#[derive(zvariant::Type)]
#[zvariant(signature = "(ua{sv})")]
#[repr(u8)]
pub enum Response<T: zvariant::Type> {
    Success(T) = 0,
    Cancelled = 1,
    Aborted = 2,
}

impl<T: zvariant::Type + serde::Serialize> serde::Serialize for Response<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success(res) => (0, res).serialize(serializer),
            Self::Cancelled => (1, HashMap::<String, zvariant::Value>::new()).serialize(serializer),
            Self::Aborted => (2, HashMap::<String, zvariant::Value>::new()).serialize(serializer),
        }
    }
}
