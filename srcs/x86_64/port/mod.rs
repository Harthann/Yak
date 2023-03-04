use core::marker::PhantomData;

mod markers;
pub use markers::{ReadOnlyAccess, WriteOnlyAccess, ReadWriteAccess};
pub use markers::{PortReadAccess, PortWriteAccess};

mod ops;
pub use ops::{PortRead, PortWrite};

/// Generic structure for a Port type.
/// Generic T: Data type to read/write (e.g: u8,u16,u32)
/// Generic MODE: Structure marker to validate access type (e.g: PortReadAccess)
///
/// Function depending on the access MODE selected will force the generic MODE to implement the
/// trait corresponding to the correct access
/// e.g: impl<T, MODE: PortReadAccess> for PortGeneric<T, MODE> ....
/// If the struct passed as geenric argument implement the correct marker trait
/// implementation will be done
pub struct PortGeneric<T, MODE> {
    port: u16,
    _mode: PhantomData<(T, MODE)>,
}

impl<T, MODE> PortGeneric<T, MODE> {
    pub const fn new(port: u16) -> Self {
        Self {
            port: port,
            _mode: PhantomData
        }
    }
}

/// Type aliases for Read/Write accessed port
pub type Port<T>            = PortGeneric<T, ReadWriteAccess>;
/// Type aliases for Read only accessed port
pub type PortReadOnly<T>    = PortGeneric<T, ReadOnlyAccess>;
/// Type aliases for Write only accessed port
pub type PortWriteOnly<T>   = PortGeneric<T, WriteOnlyAccess>;


impl<T: PortRead, MODE: PortReadAccess> PortGeneric<T, MODE> {
    pub unsafe fn read(& self) -> T {
        T::read_from_port(self.port)
    }
}

impl<T: PortWrite, MODE: PortWriteAccess> PortGeneric<T, MODE> {
    pub unsafe fn write(&mut self, value: T){
        T::write_to_port(self.port, value);
    }
}
