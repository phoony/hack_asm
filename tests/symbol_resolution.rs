#![feature(once_cell)]
use std::{
    lazy::SyncLazy,
    sync::{Mutex, MutexGuard},
};

use hack_asm::{
    self,
    instructions::{AInstruction, Compilable},
    SymbolTable, SYMBOL_TABLE,
};

static TABLE_MUTEX: SyncLazy<Mutex<()>> = SyncLazy::new(Mutex::default);

// Because tests run in parallel, we need to lock the global symbol table artificially
fn reset_and_lock_table() -> MutexGuard<'static, ()> {
    let lock = TABLE_MUTEX.lock().unwrap();
    let mut guard = SYMBOL_TABLE.write().unwrap();
    *guard = SymbolTable::default();
    lock
}

#[test]
fn symbol_undefined() {
    let _lock = reset_and_lock_table();
    let instr = AInstruction::Symbol("some_symbol");
    assert!(instr.compile().is_err());
}

#[test]
fn symbol_built_in() -> Result<(), anyhow::Error> {
    let _lock = reset_and_lock_table();
    let instr = AInstruction::Symbol("R10");
    assert_eq!(instr.compile()?, 0b0000000000001010);
    Ok(())
}

#[test]
fn symbol_user_defined() -> Result<(), anyhow::Error> {
    let _lock = reset_and_lock_table();
    SYMBOL_TABLE.write().unwrap().set("some_symbol", 42)?;
    let instr = AInstruction::Symbol("some_symbol");
    assert_eq!(instr.compile()?, 0b0000000000101010);
    Ok(())
}
