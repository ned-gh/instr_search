use std::collections::BTreeMap;

pub fn instr_search(instr: &str, map: &BTreeMap<&str, u16>) -> Option<u16> {
    map.get(instr).copied()
}
