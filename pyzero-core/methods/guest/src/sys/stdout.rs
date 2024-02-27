use risc0_zkvm::guest::env;
use rustpython_vm::{
    builtins::PyStrRef, extend_class, py_class, PyObjectRef, PyRef, PyResult, VirtualMachine,
};

pub static mut STDOUT: String = String::new();

fn stdout_impl(_self: PyObjectRef, out: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
    let out = out.as_str();

    env::write_slice(out.as_bytes());

    unsafe {
        STDOUT.push_str(out);
    }

    Ok(())
}

pub fn add_stdout_impl(vm: &VirtualMachine) {
    let ctx = &vm.ctx;

    let cls = PyRef::leak(py_class!(
        ctx,
        "RiscZeroStdout",
        vm.ctx.types.object_type.to_owned(),
        {}
    ));

    extend_class!(ctx, &cls, {
        "write" => vm.new_method("write", cls, stdout_impl),
        "flush" => vm.new_method("flush", cls, |_self: PyObjectRef| {}),
    });

    let stdout = ctx.new_base_object(cls.to_owned(), None);

    vm.sys_module.set_attr("stdout", stdout, vm).unwrap();
}
