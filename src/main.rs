mod arc_without_weak;
mod atom;
mod bif;
mod etf;
mod immix;
mod loader;
mod mailbox;
mod module;
mod module_registry;
mod opcodes;
mod pool;
mod process;
mod process_table;
mod queue;
mod value;
mod vm;

#[macro_use]
extern crate once_cell;

use time;

fn main() {
    let vm = vm::Machine::new();

    vm.start("./examples/Elixir.Test.beam");

    println!("execution time: {:?}", vm.elapsed_time())
}
