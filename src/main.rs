use wasmtime::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let engine = Engine::new(
        Config::new()
            .async_support(true)
            .wasm_threads(true)
            .debug_info(true)
            .wasm_backtrace_details(WasmBacktraceDetails::Enable),
    )
    .unwrap();
    let shared_memory = SharedMemory::new(&engine, MemoryType::shared(18, 16384)).unwrap();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(
        &engine,
        "example/target/wasm32-unknown-unknown/debug/example.wasm",
    )
    .unwrap();
    let instance = Instance::new_async(&mut store, &module, &[Extern::SharedMemory(shared_memory)])
        .await
        .unwrap();
    let math = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "math")
        .unwrap();
    let result = math.call_async(&mut store, (4, 7)).await.unwrap();
    println!("math(4, 7) = {}", result);
}
