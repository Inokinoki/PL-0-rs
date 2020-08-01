
pub enum fct {
    lit,
    opr,
    lod,
    sto,
    cal,
    inte,
    jmp,
    jpc,
}

/* instruction structure */
pub struct instruction {
    f: fct,         // instruction
    l: i32,         // level difference between declaration and reference
    a: i32,         // a variant depending on l
}
