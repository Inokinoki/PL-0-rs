
const NAME_TABLE_CAPACITY: u16 = 100;

/* types in name table */
pub enum NameTableObject {
    Constant,
    Variable,
    Procedur,
}

pub struct NameTable {
    name: String,
    kind: NameTableObject,
    val: i32,
    level: i32,
    adr: i32,
    size: i32,
}

// pub const table: Vec<table_struct> = Vec::with_capacity(NAME_TABLE_CAPACITY.into());
