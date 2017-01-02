extern crate byteorder;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Read;
use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod lookup;

struct Header {
  magic_bytes: Vec<u8>,
  class: u8,
  data_endianness: u8,
  version: u8,
  osabi: u8,
  abi_version: u8,
  binary_type: u16,
  machine: u16
}

struct ByteBuf<'a>(&'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      let l = self.0.len() - 1;
        for byte in self.0 {
          if *byte == self.0[l] {
            try!( fmtr.write_fmt(format_args!("{:#02x}", byte)));
          } else {
            try!( fmtr.write_fmt(format_args!("{:#02x} ", byte)));
          }
        }
        Ok(())
    }
}

fn parse(file: File) -> Header {
  let bytes: Vec<_> = file.bytes().map(|x| x.unwrap()).collect();

  let mut magic_bytes = vec![0; 4];
  magic_bytes[..4].copy_from_slice(&bytes[0 .. 4]);

  // endianness for reading remaining data
  let data_endianness = bytes[5]; // endianness

  let binary_type_slice = &bytes[16 .. 18];
  let mut binary_type_rdr = Cursor::new(binary_type_slice.to_vec());
  let binary_type = match data_endianness {
      0x01 => binary_type_rdr.read_u16::<LittleEndian>().unwrap(),
      0x02 => binary_type_rdr.read_u16::<BigEndian>().unwrap(),
      _    => panic!("Error: unknown endianness")
  };

  let isa_slice = &bytes[18 .. 20]; // instruction set
  let mut isa_rdr = Cursor::new(isa_slice.to_vec());
  let isa = match data_endianness {
      0x01 => isa_rdr.read_u16::<LittleEndian>().unwrap(),
      0x02 => isa_rdr.read_u16::<BigEndian>().unwrap(),
      _    => panic!("Error: unknown endianness")
  };
  Header {
    magic_bytes: magic_bytes,
    class: bytes[4], // 32-bit or 64-bit
    data_endianness: data_endianness, // endianness
    version: bytes[6],
    osabi: bytes[7],
    abi_version: bytes[8],
    // pad: &bytes[9 .. 15], // unused
    binary_type: binary_type,
    machine: isa // instruction set
  }
}

fn open_file(input: Option<String>) -> Option<File> {

  match input {

    Some(x) => {

      // Create a path to the desired file
      let path = Path::new(&x);
      let display = path.display();

      // Open the path in read-only mode, returns `io::Result<File>`
      match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => Some(file)
      }

    },
    None => {
      println!("Please enter ELF binary");
      None
    }

  }

}

fn main() {
  println!("***ELF parser***\n\nResult:");
  let input = env::args().nth(1);
  let file = open_file(input).unwrap();
  let header = parse(file);
  println!("Magic bytes: {:x}", ByteBuf(&header.magic_bytes));
  println!("Class(32- or 64-bit): {:#x}", header.class);
  println!("Data(endianness): {:#x}", header.data_endianness);
  println!("ELF Version: {:#x}", header.version);
  println!("OS ABI: {}", lookup::lookup_osabi(header.osabi));
  println!("ABI Version: {:#x}", header.abi_version);
  println!("Binary Type: {}", lookup::lookup_binary_type(header.binary_type));
  println!("Instruction Set: {}", lookup::lookup_isa(header.machine));
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
