use fnrpc::shared::ObjectIdentifier;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PoolKey(pub i32, pub i32);

impl From<&ObjectIdentifier> for PoolKey {
    fn from(value: &ObjectIdentifier) -> Self {
        Self(value.object_id, value.secondary_id)
    }
}

impl From<&PoolKey> for ObjectIdentifier {
    fn from(val: &PoolKey) -> Self {
        ObjectIdentifier {
            object_id: val.0,
            secondary_id: val.1,
        }
    }
}
