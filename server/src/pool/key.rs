use fnrpc::shared::ObjectIdentifier;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PoolKey(pub u32, pub u32);

impl From<&ObjectIdentifier> for PoolKey {
    fn from(value: &ObjectIdentifier) -> Self {
        Self(value.object_id as u32, value.secondary_id as u32)
    }
}

impl Into<ObjectIdentifier> for &PoolKey {
    fn into(self) -> ObjectIdentifier {
        ObjectIdentifier {
            object_id: self.0 as i32,
            secondary_id: self.1 as i32,
        }
    }
}
