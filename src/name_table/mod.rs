
const NAME_TABLE_CAPACITY: u16 = 100;

/* types in name table */
pub enum object {
    constant,
    variable,
    procedur,
}

pub struct table_struct {
    name: String,
    kind: object,
    val: i32,
    level: i32,
    adr: i32,
    size: i32,
}

// pub const table: Vec<table_struct> = Vec::with_capacity(NAME_TABLE_CAPACITY.into());
