pub mod stdout;

use rustpython_vm::VirtualMachine;

pub fn add_to_vm(vm: &VirtualMachine) {
    stdout::add_to_vm(vm);
}
