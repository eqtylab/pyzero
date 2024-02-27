use risc0_zkvm::sha::{Impl, Sha256};
use rustpython_vm::{
    builtins::PyBytes, extend_class, py_class, PyObjectRef, PyRef, PyResult, VirtualMachine,
};

pub fn add_to_vm(vm: &VirtualMachine) {
    let ctx = &vm.ctx;

    let cls = PyRef::leak(py_class!(
        ctx,
        "RiscZeroSha",
        vm.ctx.types.object_type.to_owned(),
        {}
    ));

    extend_class!(ctx, &cls, {
        "hash_bytes" => vm.new_method("hash_bytes", cls, hash_bytes),
    });

    let risc0 = ctx.new_base_object(cls.to_owned(), None);

    vm.sys_module.set_attr("risc0_sha", risc0, vm).unwrap();
}

fn hash_bytes(
    _self: PyObjectRef,
    data: PyRef<PyBytes>,
    vm: &VirtualMachine,
) -> PyResult<PyRef<PyBytes>> {
    let data = data.as_bytes();

    let hash = Impl::hash_bytes(data);
    let hash = hash.as_bytes().to_vec();
    let hash = vm.ctx.new_bytes(hash);

    Ok(hash)
}
