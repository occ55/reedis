use crate::common_traits::*;
use crate::data::{Data, DataType};
use crate::error::MyError;
use crate::util::*;
use std::collections::{HashMap, HashSet};

pub struct MockDatabase {
    table: Box<MockTable>,
}

impl Database for MockDatabase {
    type Table = MockTable;
    type Event = MockEvent;
    type CommandResult = MockCommandResult;

    fn table(&self) -> &Self::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Self::Table {
        &mut self.table
    }
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            table: MockTable::new(),
        }
    }
}

pub struct MockTable {
    map: HashMap<String, MockField>,
    child_listeners: usize,
}

impl Table for MockTable {
    type Field = MockField;
    type Event = MockEvent;

    fn new() -> Box<Self> {
        Box::new(MockTable {
            map: HashMap::new(),
            child_listeners: 0,
        })
    }

    fn child_listener_ct(&self) -> usize {
        self.child_listeners
    }

    fn set_child_listener_ct(&mut self, val: usize) -> usize {
        self.child_listeners = val;
        val
    }

    fn get_field(&self, key: &str) -> Option<&Self::Field> {
        self.map.get(key)
    }

    fn get_field_mut(&mut self, key: &str) -> Option<&mut Self::Field> {
        self.map.get_mut(key)
    }

    fn set_field(&mut self, key: &str, field: Self::Field) -> Result<(), MyError> {
        if !self.map.contains_key(key) {
            self.map.insert(key.to_string(), field);
            Ok(())
        } else {
            Err(MyError::KeyAlreadyExists)
        }
    }

    fn keys_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(self.map.keys().map(|e| e.as_str()))
    }
}

pub struct MockField {
    data: Data<MockTable>,
    listeners: HashSet<usize>,
    child_listeners: usize,
}

impl Field for MockField {
    type Table = MockTable;

    fn create_with_data(data: Data<Self::Table>) -> Self {
        MockField {
            data,
            listeners: HashSet::new(),
            child_listeners: 0,
        }
    }

    fn get_data(&self) -> &Data<Self::Table> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Data<Self::Table> {
        &mut self.data
    }

    fn data_type(&self) -> DataType {
        self.data.data_type()
    }

    fn add_listener(&mut self, listener: usize) {
        self.listeners.insert(listener);
    }

    fn remove_listener(&mut self, listener: usize) {
        self.listeners.remove(&listener);
    }

    fn own_listeners<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        Box::new(self.listeners.iter().map(|e| *e))
    }

    fn own_listener_ct(&self) -> usize {
        self.listeners.len()
    }

    fn child_listener_ct(&self) -> usize {
        self.child_listeners
    }

    fn set_child_listener_ct(&mut self, val: usize) -> usize {
        self.child_listeners = val;
        val
    }
}

pub struct MockEvent {
    target: usize,
    content: MockEventContent,
}

impl Event for MockEvent {
    type Content = MockEventContent;

    fn new(path: &str, op: Operation, target: usize) -> Self {
        MockEvent {
            target,
            content: MockEventContent {
                path: path.to_string(),
                operation: op,
            },
        }
    }

    fn get_target(&self) -> usize {
        self.target
    }

    fn get_content(&self) -> &Self::Content {
        &self.content
    }
}

pub struct MockEventContent {
    path: String,
    operation: Operation,
}

impl EventContent for MockEventContent {}

pub struct MockCommand {
    path: Option<String>,
    terminate: bool,
    operation: Operation,
    mutator: bool,
    args: Vec<CommandArg<MockTable>>,
}

impl Command for MockCommand {
    type Table = MockTable;

    fn get_path<'a>(&'a self) -> Option<&'a str> {
        self.path.as_deref()
    }

    fn is_mutator(&self) -> bool {
        self.mutator
    }

    fn is_terminate(&self) -> bool {
        self.terminate
    }

    fn get_operation(&self) -> Operation {
        self.operation.clone()
    }

    fn get_args_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a CommandArg<MockTable>> + 'a> {
        Box::from(self.args.iter())
    }
}

impl MockCommand {
    pub fn new_terminate() -> Self {
        MockCommand {
            path: None,
            terminate: true,
            operation: Operation::Terminate,
            mutator: false,
            args: vec![],
        }
    }

    pub fn get(path: &str) -> Self {
        MockCommand {
            path: Some(path.to_string()),
            terminate: false,
            operation: Operation::Get,
            mutator: false,
            args: vec![],
        }
    }

    pub fn set(path: &str, data: Data<MockTable>) -> Self {
        MockCommand {
            path: Some(path.to_string()),
            terminate: false,
            operation: Operation::Set,
            mutator: false,
            args: vec![CommandArg::Data(Data::Int(42))],
        }
    }
}

pub struct MockCommandResult {
    mod_count: usize,
}

impl CommandResult for MockCommandResult {
    fn modified_row_count(&self) -> usize {
        self.mod_count
    }
}