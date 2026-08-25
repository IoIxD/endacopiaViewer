use std::{
    fs::File,
    io::{BufReader, Read},
};

pub trait FromBytes: Sized {
    const SIZE: usize;
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_from_bytes {
    ($($t:ty),*) => {
        $(
            impl FromBytes for $t {
                const SIZE: usize = std::mem::size_of::<$t>();
                fn from_le_bytes(bytes: &[u8]) -> Self {
                    <$t>::from_le_bytes(bytes.try_into().unwrap())
                }
            }
        )*
    };
}

impl_from_bytes!(u8, u16, u32, u64, i8, i16, i32, i64);

pub fn read_int<T: FromBytes>(reader: &mut BufReader<File>) -> T {
    let mut buf = vec![0u8; T::SIZE];
    reader.read_exact(&mut buf).unwrap();
    T::from_le_bytes(&buf)
}
