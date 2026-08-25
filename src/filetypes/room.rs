use std::{
    ffi::{c_longlong, c_uint},
    io::{BufReader, Cursor, Read},
    os::raw::c_void,
};

use crate::{backend::Backend, readutils::read_int_cursor};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BlockType {
    Ext = 0,
    Main = 1,
    Script = 2,
    Compscript = 3,
    Compscript2 = 4,
    ObjectNames = 5,
    AnimBkgrnd = 6,
    Compscript3 = 7, /* only bytecode script type supported by released engine code */
    Properties = 8,
    ObjectScriptNames = 9,
    EOF = 0xFF,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct RoomFile {
    version: u16,
    blockpos: [u32; 10],
    blocklen: [c_uint; 10],
}

impl Backend {
    pub fn handle_crm(&self, str: String) {
        let ags = self.ags.as_ref().unwrap();
        let mut bytes = BufReader::new(Cursor::new(ags.get_file(str.clone())));

        let data_version = read_int_cursor::<u16>(&mut bytes);

        println!("{}", data_version);
        loop {
            let block_type: BlockType =
                unsafe { std::mem::transmute::<i32, _>(read_int_cursor::<u8>(&mut bytes) as i32) };
            if block_type == BlockType::Ext {
                let mut st = vec![0; 16];
                bytes.read_exact(&mut st).unwrap();
                println!("Ext: {:?}", std::str::from_utf8(&st).unwrap());
            } else {
                println!("{:?}", block_type);
            }

            if block_type == BlockType::EOF {
                break;
            }

            let block_len = {
                if data_version < 32 {
                    read_int_cursor::<i32>(&mut bytes) as i64
                } else {
                    read_int_cursor::<i64>(&mut bytes)
                }
            };

            println!("block_len {}", block_len);

            let mut dummy_buf = vec![0; block_len as usize];
            bytes.read_exact(&mut dummy_buf);
        }
    }
}
