use wasmtime::*;

fn main() {
    let engine = Engine::new(Config::new().debug_info(true).wasm_threads(true)).unwrap();
    let shared_memory = SharedMemory::new(&engine, MemoryType::shared(18, 16384)).unwrap();
    let mut store = Store::new(&engine, ());
    let module = Module::from_file(
        &engine,
        "example/target/wasm32-unknown-unknown/debug/example.wasm",
    )
    .unwrap();
    let instance =
        Instance::new(&mut store, &module, &[Extern::SharedMemory(shared_memory)]).unwrap();
    let math = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "math")
        .unwrap();
    let result = math.call(&mut store, (4, 7)).unwrap();
    println!("math(4, 7) = {}", result);
}
