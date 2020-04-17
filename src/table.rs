use crate::*;
use std::collections::hash_map::HashMap;
use std::mem::size_of;
#[derive(Debug, Clone)]
pub struct Table {
    size: usize,
    table: HashMap<String, Value>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            size: size_of::<u32>(), // kv_count size
            table: HashMap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Table {
            size: size_of::<u32>(), // kv_count size
            table: HashMap::with_capacity(capacity),
        }
    }
    pub fn with_hashmap(table: HashMap<String, Value>, size: usize) -> Self {
        Table { table, size }
    }
    pub fn get(&self, mut key: VecDeque<String>) -> Option<&Value> {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            let val = self.table.get(curr.as_str());
            if key.len() == 1 {
                val
            } else {
                match val {
                    Some(Value::Table(t2)) => t2.get(key),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, mut key: VecDeque<String>) -> Option<&mut Value> {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            let val = self.table.get_mut(curr.as_str());
            if key.len() == 1 {
                val
            } else {
                match val {
                    Some(Value::Table(t2)) => t2.get_mut(key),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
    pub fn set(&mut self, mut key: VecDeque<String>, val: &Value) {
        if key.len() > 0 {
            self.size += SIZE_USIZE + key[0].bytes().len();
            self.size += val.size();
            let curr = key.pop_front().unwrap();
            if key.len() == 0 {
                self.table.insert(curr, val.clone());
            } else {
                let existing_table = self.table.get_mut(key.get(0).unwrap());
                match existing_table {
                    Some(Value::Table(existing_table)) => {
                        existing_table.set(key, val);
                    }
                    None => {
                        let mut ntable = Table::new();
                        ntable.set(key, val);
                        self.table.insert(curr, Value::Table(Box::from(ntable)));
                    }
                    Some(_) => {
                        // ignored
                    }
                }
            }
        }
    }

    pub fn key_count(&self) -> usize {
        self.table.len()
    }
    pub fn pairs(&self) -> std::collections::hash_map::Iter<String, Value> {
        self.table.iter()
    }

    pub fn byte_size(&self) -> usize {
        self.size
    }
}
