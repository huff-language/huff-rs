use phf::phf_map;

/// Map of u32 IDs to cheat codes
pub const HUFF_CHEATS_MAP: phf::Map<u32, HuffCheatCode> = phf_map! {
    1u32 => HuffCheatCode::Log,
};

/// Huff tests cheatcodes
pub enum HuffCheatCode {
    Log,
}
