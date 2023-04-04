/// Marker to allow read access
pub trait PortReadAccess {}
/// Marker to allow write access
pub trait PortWriteAccess {}

/// Marker structure that allow only read access
pub struct ReadOnlyAccess {}
impl PortReadAccess for ReadOnlyAccess {}

/// Marker structure that allow only write access
pub struct WriteOnlyAccess {}
impl PortWriteAccess for WriteOnlyAccess {}

/// Marker structure that allow both read/write access
pub struct ReadWriteAccess {}
impl PortWriteAccess for ReadWriteAccess {}
impl PortReadAccess for ReadWriteAccess {}
