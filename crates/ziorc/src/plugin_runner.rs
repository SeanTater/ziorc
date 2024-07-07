use wasmtime::component::*;
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    inline: r#"
        package component:ziorc-plugin-example;

        /// An example world for the component to target.
        world example {
            import name: func() -> string;
            export hello-world: func() -> string;
        }
    "#,
  });


struct MyState {
    name: String,
    count: usize,
    wasi: WasiCtx,
    table: ResourceTable
}

// Imports into the world, like the `name` import for this world, are
// satisfied through traits.
impl ExampleImports for MyState {
    fn name(&mut self) -> String {
        self.name.clone()
    }
}

#[test]
pub fn test_example() -> anyhow::Result<()> {
    
    let engine = Engine::default();
    // let module = Module::new(&engine, include_bytes!("../../ziorc-plugin-hello-world/pkg/ziorc_plugin_hello_world_bg.wasm"))?;
    let component = Component::from_file(&engine, "../../target/wasm32-wasi/debug/ziorc_plugin_example.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    Example::add_to_linker(&mut linker, |state: &mut MyState| state)?;
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build();

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        MyState {
            name: "me".to_string(),
            count: 3,
            wasi,
            table: ResourceTable::new()
        },
    );
    let (bindings, _) = Example::instantiate(&mut store, &component, &linker)?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    let output = bindings.call_hello_world(&mut store)?;
    assert_eq!(output, "Hello, World!");
    Ok(())

}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.wasi }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}