#![allow(clippy::missing_transmute_annotations)]

use goblin::elf::{Elf, SectionHeader};
use std::io::Read;

use crate::ffi;

macro_rules! symbol {
    ($elf1:ident | $elf2:ident, $base:ident, $name:expr) => {
        unsafe {
            std::mem::transmute(match symbol_by_name(&$elf1, $base, $name) {
                Ok(sym) => sym,
                Err(e) => {
                    if let Some(ref elf2) = $elf2 {
                        symbol_by_name(elf2, $base, $name)?
                    } else {
                        return Err(e);
                    }
                }
            })
        }
    };
}

pub struct ArtSymbolTable {
    pub runtime_instance: ffi::RuntimeInstance,
    pub ensure_plugin_loaded: ffi::EnsurePluginLoadedFn,
}

impl ArtSymbolTable {
    pub unsafe fn from_libart() -> crate::Result<Self> {
        // Get the module path and base address of libart.
        let (module_path, base_addr) = module_info("libart.so")?;
        // Parse the ELF image of libart.
        let elf1_data = std::fs::read(module_path)?;
        let elf1 = Elf::parse(&elf1_data)?;
        // Parse the decompressed symtab of libart if it exists.
        let elf2_data = section_by_name(&elf1, ".gnu_debugdata")
            .map(|hdr| decompress_debuginfo(&elf1_data, hdr))
            .transpose()?;
        let elf2 = elf2_data.as_ref().map(|d| Elf::parse(d)).transpose()?;
        // Create the symbol table.
        Ok(Self {
            runtime_instance: symbol!(elf1 | elf2, base_addr, ffi::sym::RUNTIME_INSTANCE),
            ensure_plugin_loaded: symbol!(elf1 | elf2, base_addr, ffi::sym::ENSURE_PLUGIN_LOADED),
        })
    }
}

fn module_info(module: &str) -> crate::Result<(String, u64)> {
    let maps = std::fs::read_to_string("/proc/self/maps")?;
    for line in maps.lines() {
        if !line.contains("r-xp") && !line.contains("r--p") {
            continue;
        }
        let mut line = line.split_whitespace();
        let Some(range) = line.next() else {
            continue;
        };
        let Some(path) = line.last() else {
            continue;
        };
        if !path.contains(module) {
            continue;
        };
        let Some((start, _)) = range.split_once('-') else {
            continue;
        };
        let Ok(start) = u64::from_str_radix(start.trim_start_matches("0x"), 16) else {
            continue;
        };
        return Ok((path.into(), start));
    }

    Err(crate::Error::ModuleNotFound)
}

fn section_by_name<'a>(elf: &'a Elf<'_>, name: &str) -> Option<&'a SectionHeader> {
    elf.section_headers
        .iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name) == Some(name))
}

fn decompress_debuginfo(buf: &[u8], hdr: &SectionHeader) -> crate::Result<Vec<u8>> {
    let compressed_data = &buf[hdr.file_range().unwrap()];
    let mut decompressed_data = Vec::new();
    xz2::read::XzDecoder::new(compressed_data).read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

fn symbol_by_name(elf: &Elf<'_>, base: u64, name: &str) -> crate::Result<u64> {
    let Some(sym) = elf
        .syms
        .iter()
        .find(|sym| elf.strtab.get_at(sym.st_name) == Some(name))
    else {
        return Err(crate::Error::SymbolNotFound(name.into()));
    };
    Ok(base.saturating_add(sym.st_value))
}
