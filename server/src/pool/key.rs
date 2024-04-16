use fnrpc::shared::ObjectIdentifier;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PoolKey(pub i32, pub i32);

impl From<&ObjectIdentifier> for PoolKey {
    fn from(value: &ObjectIdentifier) -> Self {
        Self(value.object_id, value.secondary_id)
    }
}

impl Into<ObjectIdentifier> for &PoolKey {
    fn into(self) -> ObjectIdentifier {
        ObjectIdentifier {
            object_id: self.0,
            secondary_id: self.1,
        }
    }
}
