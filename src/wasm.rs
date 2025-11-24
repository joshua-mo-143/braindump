//! A module that deals primarily with WASM compatibility.

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub trait WasmCompatSend: Send {}

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
impl<T> WasmCompatSend for T where T: Send {}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub trait WasmCompatSend {}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl<T> WasmCompatSend for T {}

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub trait WasmCompatSync: Sync {}

#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
impl<T> WasmCompatSync for T where T: Sync {}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub trait WasmCompatSync {}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl<T> WasmCompatSync for T {}
