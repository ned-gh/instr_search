use rustc_hash::FxHashMap;

pub fn instr_search(instr: &str, map: &FxHashMap<&str, u16>) -> Option<u16> {
    map.get(instr).copied()
}
