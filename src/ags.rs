use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    os::raw::c_int,
};

use parking_lot::Mutex;

trait FromBytes: Sized {
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

fn read_int<T: FromBytes>(reader: &mut BufReader<File>) -> T {
    let mut buf = vec![0u8; T::SIZE];
    reader.read_exact(&mut buf).unwrap();
    T::from_le_bytes(&buf)
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    filename: String,
    _file_datafile: u8,
    offset: u64,
    length: u64,
}

impl FileInfo {
    pub fn filename(&self) -> String {
        self.filename.clone()
    }
    pub fn offset(&self) -> u64 {
        self.offset
    }
    pub fn length(&self) -> u64 {
        self.length
    }
}

pub struct AGS {
    reader: Mutex<BufReader<File>>,
    normal_files: HashMap<String, FileInfo>,
    begseek: u64,
}

impl AGS {
    pub fn new() -> Self {
        let mut reader = BufReader::new(File::open("./Endacopia.exe").unwrap());

        /* At the end of an exe is an offset to a "CLIB" block that has the information we want */
        reader.seek(SeekFrom::End(-20)).unwrap();

        let begseek = read_int::<u64>(&mut reader);
        reader.seek(SeekFrom::Start(begseek)).unwrap();

        /* verify header */
        let mut magicheader = [0; 5];
        reader.read_exact(&mut magicheader).unwrap();
        println!("{:?}", magicheader);
        assert_eq!(magicheader, [67, 76, 73, 66, 26]);

        /* lib version */
        let libversion = read_int::<u8>(&mut reader);

        assert_eq!(read_int::<u8>(&mut reader), 0);

        let mut s = Self {
            reader: Mutex::new(reader),
            normal_files: HashMap::new(),
            begseek,
        };

        /* If I were to ever expand this for other AGS games, this is where I'd support more formats */
        if libversion >= 30 {
            s.decode_v30();
        } else {
            panic!("Unsupported libversion {}", libversion);
        }

        s
    }

    fn decode_v30(&mut self) {
        let mut reader = self.reader.lock();
        /* skipping */
        read_int::<c_int>(&mut reader);

        /* "data files" get ignored for now */
        let num_data_files = read_int::<c_int>(&mut reader);
        println!("{:?} data files found", num_data_files);
        for _ in 0..num_data_files {
            let mut buf = vec![];
            reader.read_until(0, &mut buf).unwrap();
            println!("{:?}", str::from_utf8(&buf));
        }

        let num_files = read_int::<c_int>(&mut reader);
        println!("{:?} normal files found", num_files);
        for _ in 0..num_files {
            let mut buf = vec![];
            reader.read_until(0, &mut buf).unwrap();
            let file_datafile = read_int::<u8>(&mut reader);
            let offset = read_int::<u64>(&mut reader);
            let length = read_int::<u64>(&mut reader);
            if let Ok(filename) = str::from_utf8(&buf) {
                let filename = String::from(filename);
                let info = FileInfo {
                    filename: filename.clone(),
                    _file_datafile: file_datafile,
                    offset,
                    length,
                };
                self.normal_files.insert(filename, info);
            }
        }
    }

    pub fn normal_files(&self) -> HashMap<String, FileInfo> {
        self.normal_files.clone()
    }

    pub fn get_file(&self, filename: String) -> Vec<u8> {
        let mut reader = self.reader.lock();

        let file = self.normal_files.get(&filename).unwrap();

        reader
            .seek(SeekFrom::Start(self.begseek + file.offset()))
            .unwrap();

        let mut buf = Vec::new();
        buf.resize(file.length() as usize, 0);
        reader.read_exact(&mut buf).unwrap();

        return buf;
    }
}
