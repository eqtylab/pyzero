use risc0_zkvm::guest::env;
use rustpython_vm::{
    builtins::PyStrRef, extend_class, py_class, PyObjectRef, PyRef, PyResult, VirtualMachine,
};

static mut STDOUT: String = String::new();

pub fn get_string() -> String {
    // TODO: find a good way to not duplicate the stdout string
    // maybe just commit stdout as we go
    unsafe { STDOUT.clone() }
}

pub fn add_to_vm(vm: &VirtualMachine) {
    let ctx = &vm.ctx;

    let cls = PyRef::leak(py_class!(
        ctx,
        "RiscZeroStdout",
        vm.ctx.types.object_type.to_owned(),
        {}
    ));

    extend_class!(ctx, &cls, {
        "write" => vm.new_method("write", cls, write),
        "flush" => vm.new_method("flush", cls, flush),
    });

    let stdout = ctx.new_base_object(cls.to_owned(), None);

    vm.sys_module.set_attr("stdout", stdout, vm).unwrap();
}

fn write(_self: PyObjectRef, out: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
    let out = out.as_str();

    env::write_slice(out.as_bytes());

    unsafe {
        STDOUT.push_str(out);
    }

    Ok(())
}

fn flush(_self: PyObjectRef) {
    // do nothing
}
