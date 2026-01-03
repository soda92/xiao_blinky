# nRF52840 Debugging Guide: From Xiao to DK

This guide covers how to transition from "blind" debugging on the Seeed Xiao nRF52840 (via USB bootloader) to professional-grade debugging using the nRF52840 Development Kit (DK) and J-Link.

## 1. Why use the DK?

The Seeed Xiao is great for deployment, but debugging via the UF2 bootloader has limits:
*   **No Real-time Logs:** You can't see logs if the USB stack crashes or hasn't started.
*   **No Breakpoints:** You can't pause execution to inspect variables.
*   **Silent Failures:** HardFaults and Panics often just look like a "frozen" LED.

The nRF52840 DK includes an onboard **J-Link debugger**, enabling:
*   **RTT (Real-Time Transfer):** Extremely fast logging that persists even through crashes.
*   **Interactive Debugging:** Step-through code, inspect memory, and view call stacks.
*   **Panic Traces:** Automatic printing of panic locations (file/line).

## 2. Hardware Setup

### Debugging the DK Itself
Simply connect the DK to your PC via the USB connector marked **J-Link**.

### Debugging the Xiao using the DK
You can use the DK to debug the Xiao! The DK automatically detects external targets.
1.  **Power the Xiao:** Connect the Xiao to USB (or battery) so it has power.
2.  **Connect SWD Lines:** Wire the Xiao to the **Debug Out (P19)** header on the DK:
    *   **VTG** (Target Voltage) -> Xiao **3V3** (This tells the J-Link a target is present)
    *   **SWDIO** -> Xiao **D** (Bottom pad)
    *   **SWCLK** -> Xiao **C** (Bottom pad)
    *   **GND** -> Xiao **GND** (detectable common ground is essential)

## 3. Software Tools

Ensure you have the modern Rust embedded toolkit:

```bash
# Install probe-rs (Flashing & RTT)
cargo install probe-rs --features cli

# Install GDB (Interactive Debugging)
# Linux:
sudo apt install gdb-multiarch
# macOS:
brew install arm-none-eabi-gdb
```

## 4. Project Configuration (Cargo.toml & config.toml)

### Switch Runner to probe-rs
Modify `.cargo/config.toml` to use `probe-rs` instead of `hf2` or `uf2conv`.

```toml
[target.thumbv7em-none-eabihf]
# Standard run command
runner = "probe-rs run --chip nRF52840_xxAA"

# If the board is sleeping or locked, use this to force connection:
# runner = "probe-rs run --chip nRF52840_xxAA --connect-under-reset"
```

### Memory Layout (memory.x)
*   **With SoftDevice (BLE):** Keep the offset.
    *   `FLASH : ORIGIN = 0x27000`
    *   `RAM : ORIGIN = 0x20007C00`
*   **Bare Metal (No BLE):** Reset to zero.
    *   `FLASH : ORIGIN = 0x00000`
    *   `RAM : ORIGIN = 0x20000000`

## 5. Workflow A: RTT Logging (The "printf" method)

This is the daily driver workflow.

1.  **Run:**
    ```bash
    cargo run --release
    ```
2.  **Result:**
    *   Code compiles.
    *   Flashes instantly via J-Link.
    *   Terminal automatically attaches to RTT.
    *   You see logs like: `INFO [main] Starting Blinky...`

## 6. Workflow B: Interactive Debugging (GDB)

Use this for hard crashes or logic bugs.

1.  **Start GDB Server:**
    Open a terminal and run:
    ```bash
    probe-rs gdb --chip nRF52840_xxAA
    ```
    *(It listens on localhost:1337)*

2.  **Connect GDB:**
    Open another terminal in your project root:
    ```bash
    gdb-multiarch target/thumbv7em-none-eabihf/release/xiao_blinky
    ```

3.  **Commands inside GDB:**
    ```gdb
    target remote :1337    # Connect to probe-rs
    monitor reset halt     # Reset chip and stop
    break main             # Set breakpoint at main
    continue               # Run until main
    next                   # Step over
    step                   # Step into
    print my_variable      # Inspect value
    bt                     # Backtrace (Call stack)
    ```

## 7. The SoftDevice Caveat

Debugging with Bluetooth (SoftDevice) active is tricky.
*   **The Issue:** The SoftDevice needs strict timing. If you hit a breakpoint, the CPU stops, timing is lost, and the SoftDevice will crash (assert) immediately upon resuming.
*   **Strategy:** Debug your application logic *before* enabling the SoftDevice, or debug non-BLE parts. If you break in a BLE loop, expect to reset the board to reconnect.
