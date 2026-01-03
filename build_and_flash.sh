#!/bin/bash
set -e

APP_NAME="xiao_blinky"
TARGET="thumbv7em-none-eabihf"
UF2_FAMILY_ID="0xADA52840" # nRF52840 family ID
BOOTLOADER_MOUNT="/run/media/soda/XIAO-SENSE"

echo "Building $APP_NAME..."
cargo build --release

echo "Converting to Binary..."
arm-none-eabi-objcopy -O binary "target/$TARGET/release/$APP_NAME" "$APP_NAME.bin"

echo "Converting to UF2..."
python3 uf2conv.py -c -f "$UF2_FAMILY_ID" -b 0x27000 -o "$APP_NAME.uf2" "$APP_NAME.bin"

echo "Success! UF2 file created at: ./$APP_NAME.uf2"

if [ -d "$BOOTLOADER_MOUNT" ]; then
    echo "Bootloader drive found at $BOOTLOADER_MOUNT. Copying..."
    cp "$APP_NAME.uf2" "$BOOTLOADER_MOUNT/"
    echo "Flashing complete."
else
    echo "Bootloader drive not found at $BOOTLOADER_MOUNT."
    echo "Please double-tap Reset on the board and copy '$APP_NAME.uf2' to the drive manually."
fi
