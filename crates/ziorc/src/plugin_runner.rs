use wasmer::{imports, Instance, Module, Store};

#[test]
pub fn test_example() -> anyhow::Result<()> {
    
    let mut store = Store::default();
    let module = Module::new(&store, include_bytes!("../../ziorc-plugin-hello-world/pkg/ziorc_plugin_hello_world_bg.wasm"))?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let alert = instance.exports.get_typed_function::<String, ()>(&mut store, "alert")?;
    let _result = alert.call(&mut store, "Sean".into())?;

    Ok(())

}