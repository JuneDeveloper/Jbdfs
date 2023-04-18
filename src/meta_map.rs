use std::collections::HashMap;

pub(crate) struct MetaMap {
    pub(crate) structure : HashMap<String, i128>,
}

impl MetaMap {
    pub fn add(mut self, id : i128, line : String) {
        self.structure.insert(line.replace("\n", ""), id);
    }
    pub fn new() -> MetaMap {
        return MetaMap { structure : HashMap::default() };
    }

}