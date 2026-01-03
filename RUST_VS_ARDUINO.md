# Rust (Embassy) vs Arduino (Bluefruit) Comparison

## 1. Concurrency Model

**Rust (Async/Await):**
```rust
// Truly concurrent tasks
let gatt_fut = gatt_server::run(...);
let blink_fut = async { ... loop { ... Timer::after(...) ... } };

// Join them
match select(gatt_fut, blink_fut).await { ... }
```
*   **Pros:** The blink logic looks like a simple linear script (`red`, `delay`, `green`, `delay`). The compiler handles the multitasking state machine.
*   **Cons:** Requires understanding `async`, `Future`, and `spawner`.

**Arduino (Superloop):**
```cpp
// Manual state machine
if (currentMillis - previousMillis >= interval) {
    step++;
    if (step == 0) { ... }
}
```
*   **Pros:** Very easy to start with. No complex types.
*   **Cons:** Logic gets fragmented. If you add a button debounce or a sensor read, you have to manually interleave their timing checks. "Spaghetti code" risk is high.

## 2. Safety

**Rust:**
*   **Memory:** Zero-cost abstractions. `heapless::Vec` ensures you never overflow the BLE buffer.
*   **Interrupts:** `embassy-nrf` guarantees at compile time that you aren't sharing the same peripheral (e.g., Timer0) between two drivers.
*   **Panic:** If something goes wrong, you get a strict panic (and stack trace with `probe-rs`).

**Arduino:**
*   **Memory:** Uses C++ `String` or raw pointers. Easier to write, but easier to crash with heap fragmentation or buffer overflows.
*   **Hardware:** It's possible to accidentally configure two libraries to use the same timer, causing weird bugs at runtime.

## 3. Bluetooth Stack

**Rust (`nrf-softdevice`):**
*   Direct, safe bindings to the S140 SoftDevice.
*   You define the GATT table struct in Rust code (using macros).
*   Gives you immense control but requires you to understand BLE concepts (Services, Characteristics, UUIDs).

**Arduino (`Adafruit_BluefruitLE_nRF52`):**
*   Wraps everything in high-level objects (`BLEUart`, `BLEHidAdafruit`).
*   Hides the GATT table details.
*   "Just works" for standard cases, but harder to customize if you need a weird specific BLE feature not exposed by the library.

## Conclusion

*   **Arduino** is better for **prototyping** if you don't know BLE well. "I just want a UART."
*   **Rust Embassy** is better for **products**. It gives you robust concurrency, safety, and fine-grained power control out of the box.
