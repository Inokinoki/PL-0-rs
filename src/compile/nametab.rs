const NAME_TABLE_CAPACITY: u16 = 100;

/* types in name table */
#[derive(PartialEq)]
pub enum NameTableObject {
    Constant,
    Variable,
    Procedur,
}

pub struct NameTableItem {
    pub name: String,
    pub kind: NameTableObject,
    pub val: i64,
    pub level: usize,
    pub adr: usize,
    pub size: usize,
}
