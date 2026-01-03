# MicroPython Blinky for Seeed Xiao nRF52840 Sense

This project contains a simple MicroPython script to blink the RGB LED on the Seeed Xiao nRF52840 Sense.

## Prerequisites

1.  **MicroPython Firmware**: You need to have MicroPython installed on your board.
    *   Download the `.uf2` firmware for "Seeed XIAO nRF52840 Sense" from [micropython.org](https://micropython.org/download/SEEED_XIAO_NRF52840_SENSE/).
    *   Connect the board to your computer via USB.
    *   Double-press the **Reset** button on the board quickly to enter bootloader mode. A drive named `XIAO-SENSE` (or similar) should appear.
    *   Drag and drop the `.uf2` file into this drive. The board will restart.

## Deployment

1.  **Mount the Drive**: Once MicroPython is running, the board should appear as a USB mass storage device (often named `NO NAME` or similar, usually much smaller than the bootloader drive).
    *   *Note: Recent MicroPython versions for nRF52 might not expose the filesystem as a USB drive by default depending on the build. If it doesn't appear, you might need a tool like `mpremote` or `ampy`.*

2.  **Copy the Script**:
    *   If the USB drive appears: Simply copy `main.py` to the root of that drive.
    *   Using `mpremote` (recommended):
        ```bash
        pip install mpremote
        mpremote connect <port> cp main.py :main.py
        mpremote repl
        ```

3.  **Run**: 
    *   The board will automatically run `main.py` on boot.
    *   To restart the script manually, press `Ctrl+D` in the REPL or press the Reset button.

## Pinout Info
-   Red LED: P0.26
-   Green LED: P0.30
-   Blue LED: P0.06
-   Active Level: LOW (0 to turn on)
