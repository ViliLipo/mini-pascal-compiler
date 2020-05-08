use std::collections::HashMap;
use crate::ast::NodeType;



pub enum ConstructCategory {
    SimpleVar,
    ArrayVar,
    Function,
    Program,
    TypeId,
}

impl Copy for ConstructCategory {}
impl Clone for ConstructCategory {
    fn clone(&self) -> ConstructCategory {
        *self
    }
}

pub struct Entry {
    pub name: String,
    pub category: ConstructCategory,
    pub value: String,
    pub entry_type: NodeType,
    pub scope_number: i32,
    pub address: String,
}

impl Clone for Entry {
    fn clone(&self) -> Entry {
        Entry{
            name: self.name.clone(),
            category: self.category.clone(),
            value: self.value.clone(),
            entry_type: self.entry_type.clone(),
            scope_number: self.scope_number,
            address: self.address.clone(),
        }
    }
}

pub struct Scope {
    pub scope_number: i32,
    pub enclosing_scope_number: i32,
    pub is_closed: bool,
}

impl Copy for Scope {}

impl Clone for Scope {
    fn clone(&self) -> Scope {
        *self
    }
}

pub struct Symboltable {
    scopestack: Vec<i32>,
    table: HashMap<i32, HashMap<String, Entry>>,
    scope_information_table: HashMap<i32, Scope>,
    generator_no: i32,
}

impl Clone for Symboltable {
    fn clone(&self) -> Symboltable {
        Symboltable {
            scopestack: vec![0],
            table: self.table.clone(),
            scope_information_table: self.scope_information_table.clone(),
            generator_no: 1,
        }
    }
}
impl Symboltable {
    fn get_new_scope_number(&mut self) -> i32{
        let no = self.generator_no;
        self.generator_no = self.generator_no + 1;
        no
    }
    pub fn enter_scope(&mut self, scope: Scope) {
        self.scope_information_table
            .insert(scope.scope_number, scope.clone());
        self.scopestack.push(scope.scope_number);
    }

    pub fn enter_scope_with_number(&mut self, scope_number: i32) {
        if let Some(scope) = self.scope_information_table.get(&scope_number) {
            self.scopestack.push(scope.scope_number);
        }
    }

    pub fn current_scope(&self) -> Option<Scope> {
        match self.scopestack.last() {
            Some(scope_number) => match self.scope_information_table.get(scope_number) {
                Some(scope) => Some(*scope),
                None => None,
            },
            None => None,
        }
    }

    pub fn new_scope_in_current_scope(&mut self, is_closed: bool) -> i32{
        match self.scopestack.last() {
            Some(scope_number) => {
                let enclosing_scope_number = scope_number.clone();
                let no = self.get_new_scope_number();
                let scope = Scope {
                    enclosing_scope_number,
                    scope_number: no,
                    is_closed,
                };
                self.table.insert(scope.scope_number, HashMap::new());
                self.enter_scope(scope);
                no
            }
            None => -1,
        }
    }

    pub fn exit_scope(&mut self) {
        self.scopestack.pop();
    }

    pub fn add_entry(&mut self, entry: Entry) {
        match self.table.get_mut(&entry.scope_number) {
            Some(scope) => {
                scope.insert(entry.name.clone(), entry);
            }
            None => (),
        }
    }

    pub fn lookup(&self, name: &String) -> Option<&Entry> {
        let mut scope_number: Option<&i32> = self.scopestack.last();
        loop {
            match scope_number {
                Some(scope_no) => match self.lookup_explicit_scope(name, *scope_no) {
                    Some(entry) => return Some(entry),
                    None => {
                        match self.scope_information_table.get(scope_no) {
                            Some(scope) => {
                                scope_number = Some(&scope.enclosing_scope_number);
                            }
                            None => return None,
                        };
                    }
                },
                None => return None,
            }
        }
    }

    pub fn is_visible(&self, name: &String) -> bool {
        match self.lookup(name) {
            Some(_scope) => true,
            None => false,
        }
    }

    fn lookup_explicit_scope(&self, name: &String, scope_number: i32) -> Option<&Entry> {
        match self.table.get(&scope_number) {
            Some(scope) => match scope.get(name) {
                Some(entry) => Some(entry),
                None => None,
            },
            None => None,
        }
    }

    pub fn in_current_scope(&self, name: &String) -> bool {
        match self.scopestack.last() {
            Some(scope) => match self.lookup_explicit_scope(name, *scope) {
                Some(_entry) => true,
                None => false,
            },
            None => false,
        }
    }
}

fn predefined_ids() -> Vec<Entry> {
    let mut entries = Vec::new();
    entries.push(Entry {
        name: String::from("Boolean"),
        category: ConstructCategory::TypeId,
        value: String::from("0"),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("integer"),
        category: ConstructCategory::TypeId,
        value: String::from("0"),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("real"),
        category: ConstructCategory::TypeId,
        value: String::from("0"),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("string"),
        category: ConstructCategory::TypeId,
        value: String::from(""),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("false"),
        category: ConstructCategory::SimpleVar,
        value: String::from("0"),
        entry_type: NodeType::Simple(String::from("Boolean")),
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("true"),
        category: ConstructCategory::SimpleVar,
        value: String::from("1"),
        entry_type: NodeType::Simple(String::from("Boolean")),
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("writeln"),
        category: ConstructCategory::Function,
        value: String::from(""),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries.push(Entry {
        name: String::from("read"),
        category: ConstructCategory::Function,
        value: String::from(""),
        entry_type: NodeType::Unit,
        scope_number: 0,
        address: String::new(),
    });
    entries
}

pub fn get_symbol_table() -> Symboltable {
    let scope_zero_info = Scope {
        scope_number: 0,
        enclosing_scope_number: -1,
        is_closed: false,
    };
    let mut scope_zero: HashMap<String, Entry> = HashMap::new();
    for entry in predefined_ids() {
        scope_zero.insert(entry.name.clone(), entry);
    }
    let mut table: HashMap<i32, HashMap<String, Entry>> = HashMap::new();
    let mut scope_table: HashMap<i32, Scope> = HashMap::new();
    table.insert(0, scope_zero);
    scope_table.insert(0, scope_zero_info);
    Symboltable {
        scope_information_table: scope_table,
        table,
        scopestack: vec![0],
        generator_no: 1,
    }
}
