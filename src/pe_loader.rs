use goblin::pe::PE;
use std::fs;

pub struct WinProcess {
    pub entry_point: u64,
    pub image_base: u64,
    pub info_summary: String,
}

impl WinProcess {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let buffer = fs::read(path)?;
        let pe = PE::parse(&buffer)?;

        let image_base = pe.header.optional_header
            .map(|oh| oh.windows_fields.image_base)
            .unwrap_or(0x400000) as u64;

        let entry_point = image_base + pe.entry as u64;
        let info_summary = format!("PE Cargado | Base: 0x{:X} | Entry: 0x{:X}", image_base, entry_point);

        Ok(Self {
            entry_point,
            image_base,
            info_summary,
        })
    }
}