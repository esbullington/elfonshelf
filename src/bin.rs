extern crate byteorder;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Read;

mod lookup;

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

fn main() {
  println!("***ELF parser***\n\nResult:");
  let input = env::args().nth(1);
  match input {
    Some(x) => {

      // Create a path to the desired file
      let path = Path::new(&x);
      let display = path.display();

      // Open the path in read-only mode, returns `io::Result<File>`
      match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => {
          let bytes: Vec<_> = file.bytes().map(|x| x.unwrap()).collect();
          let magic_bytes = &bytes[0 .. 4];
          println!("Magic bytes: {:x}", ByteBuf(magic_bytes));
          let class = &bytes[4]; // 32-bit or 64-bit
          println!("Class(32- or 64-bit): {:#x}", class);
          let data_endianness = &bytes[5]; // endianness
          println!("Data(endianness): {:#x}", data_endianness);
          let version = &bytes[6];
          println!("ELF Version: {:#x}", version);
          let osabi = &bytes[7];
          println!("OS ABI: {}", lookup::lookup_osabi(osabi));
          let abi_version = &bytes[8];
          println!("ABI Version: {:#x}", abi_version);
          // let pad = &bytes[9 .. 15]; // unused
          let binary_type_slice = &bytes[16 .. 18];
          println!("Binary Type: {}", lookup::lookup_binary_type(&binary_type_slice, data_endianness));
          let instruction_set_slice = &bytes[18 .. 20];
          println!("Instruction Set: {}", lookup::lookup_isa(&instruction_set_slice, data_endianness));
        }
      };
    }
    None => {
      println!("Please enter ELF binary")
    }
  }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
