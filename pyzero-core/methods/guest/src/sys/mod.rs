pub mod risc0;
pub mod stdout;

use rustpython_vm::VirtualMachine;

pub fn add_to_vm(vm: &VirtualMachine) {
    risc0::add_to_vm(vm);
    stdout::add_to_vm(vm);
}
