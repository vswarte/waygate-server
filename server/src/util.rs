use std::io;
use std::mem;
use std::slice;

pub fn read_as_type<T: Sized + Default>(reader: &mut impl io::Read) -> Result<T, io::Error> {
    let result = T::default();

    let buffer: &mut [u8] = unsafe {
        slice::from_raw_parts_mut(
            &result as *const T as *const u8 as *mut u8,
            mem::size_of::<T>(),
        )
    };

    reader.read_exact(buffer)?;

    Ok(result)
}
