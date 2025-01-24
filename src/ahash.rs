use ahash::AHashMap;

pub fn instr_search(instr: &str, map: &AHashMap<&str, u16>) -> Option<u16> {
    map.get(instr).copied()
}
