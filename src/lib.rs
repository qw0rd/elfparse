#![no_std]

#[repr(C)]
#[derive(Default, Debug)]
pub struct ElfHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(u16)]
#[derive(Debug)]
pub enum ElfObjectFileType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Loos,
    Hioos,
    Loproc,
    Hiproc,
}

impl ElfObjectFileType {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0x0 => ElfObjectFileType::None,
            0x1 => ElfObjectFileType::Rel,
            0x2 => ElfObjectFileType::Exec,
            0x3 => ElfObjectFileType::Dyn,
            0x4 => ElfObjectFileType::Core,
            0xFE00 => ElfObjectFileType::Loos,
            0xFEFF => ElfObjectFileType::Hioos,
            0xFF00 => ElfObjectFileType::Loproc,
            0xFFFF => ElfObjectFileType::Hiproc,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ElfHeaderParseError {
    InvalidMagicBytes,
}

impl ElfHeader {
    pub fn from_bytes(data: &[u8]) -> Result<Self, ElfHeaderParseError> {
        let mut header = Self::default();
        let mut cur = 16;

        &header.e_ident.copy_from_slice(&data[0..cur]);
        if header.e_ident[..4].cmp(&[0x7f, 69, 76, 70]) != core::cmp::Ordering::Equal {
            return Err(ElfHeaderParseError::InvalidMagicBytes);
        }

        header.e_type = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_machine = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_version = u32::from_le_bytes(data[cur..cur + 4].try_into().unwrap());
        cur += 4;

        header.e_entry = usize::from_le_bytes(data[cur..cur + 8].try_into().unwrap());
        cur += 8;

        header.e_phoff = usize::from_le_bytes(data[cur..cur + 8].try_into().unwrap());
        cur += 8;

        header.e_shoff = usize::from_le_bytes(data[cur..cur + 8].try_into().unwrap());
        cur += 8;

        header.e_flags = u32::from_le_bytes(data[cur..cur + 4].try_into().unwrap());
        cur += 4;

        header.e_ehsize = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_phentsize = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_phnum = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_shentsize = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_shnum = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        header.e_shstrndx = u16::from_le_bytes(data[cur..cur + 2].try_into().unwrap());
        cur += 2;

        Ok(header)
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ElfSection {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

pub struct SectionIter<'a> {
    start: &'a [u8],
    size: usize,
    current: usize
}

impl<'a> Iterator for SectionIter<'a> {
    type Item = ElfSection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == self.current {
            return None;
        }
        let mut section = ElfSection::default();
        section.sh_name = u32::from_le_bytes(self.start[..4].try_into().unwrap());
        self.start = &self.start[4..];
        section.sh_type = u32::from_le_bytes(self.start[..4].try_into().unwrap());
        self.start = &self.start[4..];
        section.sh_flags = u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];
        section.sh_addr = u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];
        section.sh_offset = u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];
        section.sh_size = u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];
        section.sh_link = u32::from_le_bytes(self.start[..4].try_into().unwrap());
        self.start = &self.start[4..];
        section.sh_info = u32::from_le_bytes(self.start[..4].try_into().unwrap());
        self.start = &self.start[4..];
        section.sh_addralign= u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];
        section.sh_entsize = u64::from_le_bytes(self.start[..8].try_into().unwrap());
        self.start = &self.start[8..];

        self.current += 1;

        Some(section)
    }
}

#[derive(Debug)]
pub struct ElfFile<'a> {
    pub header: ElfHeader,
    data: &'a [u8],
}

impl<'a> ElfFile<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Result<Self, ElfHeaderParseError> {
        let mut header = ElfHeader::from_bytes(data)?;
        Ok(Self { header, data })
    }

    pub fn sections(&self) -> SectionIter {
        SectionIter {
            start: &self.data[self.header.e_shoff..],
            size: self.header.e_shnum as usize,
            current: 0
        }
    }

    pub fn section_name(&self, section: &ElfSection) -> Option<&'a str> {
        let section_table = self.sections().nth(self.header.e_shstrndx as usize)?;
        let name_at = (section_table.sh_offset as usize) + (section.sh_name as usize);
        let mut cur = name_at;

        while self.data[cur] != 0 {
            cur += 1;
        }

        Some(core::str::from_utf8(&self.data[name_at..cur]).unwrap())
    }
}
