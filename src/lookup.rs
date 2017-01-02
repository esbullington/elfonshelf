
pub fn lookup_osabi(byte: u8) -> &'static str {
  match byte {
    0x00 => "System V",
    0x01 => "HP-UX",
    0x02 => "NetBSD",
    0x03 => "Linux",
    0x06 => "Solaris",
    0x07 => "AIX",
    0x08 => "IRIX",
    0x09 => "FreeBSD",
    0x0C => "OpenBSD",
    0x0D => "OpenVMS",
    0x0E => "NonStop Kernel",
    0x0F => "AROS",
    0x10 => "Fenix OS",
    0x11 => "CloudABI",
    0x53 => "Sortix",
    _    => "Unknown"
  }
}

pub fn lookup_binary_type(binary_type: u16) -> &'static str {
  match binary_type {
    1 => "relocatable",
    2 => "executable",
    3 => "shared",
    4 => "core",
    _    => "Unknown"
  }
}

pub fn lookup_isa(isa: u16) -> &'static str {
  match isa {
    0x00 => "No specific instruction set",
    0x02 => "SPARC",
    0x03 => "x86",
    0x08 => "MIPS",
    0x14 => "PowerPC",
    0x28 => "ARM",
    0x2A => "SuperH",
    0x32 => "IA-64",
    0x3E => "x86-64",
    0xB7 => "AArch64",
    _    =>  "Unknown"
  }
}
