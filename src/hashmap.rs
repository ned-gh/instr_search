use std::collections::HashMap;

pub fn instr_search(instr: &str, map: &HashMap<&str, u16>) -> Option<u16> {
    map.get(instr).copied()
}
