import machine
import time

# LED Pins for Seeed Xiao nRF52840 Sense
# Active LOW (0 = ON, 1 = OFF)
# P0.26 -> 26, P0.30 -> 30, P0.06 -> 6
LED_RED   = machine.Pin(26, machine.Pin.OUT)
LED_GREEN = machine.Pin(30, machine.Pin.OUT)
LED_BLUE  = machine.Pin(6, machine.Pin.OUT)

def turn_off_all():
    LED_RED.value(1)
    LED_GREEN.value(1)
    LED_BLUE.value(1)

def blink_cycle(delay=0.5):
    while True:
        # Red
        turn_off_all()
        LED_RED.value(0)
        time.sleep(delay)
        
        # Green
        turn_off_all()
        LED_GREEN.value(0)
        time.sleep(delay)
        
        # Blue
        turn_off_all()
        LED_BLUE.value(0)
        time.sleep(delay)

if __name__ == "__main__":
    print("Starting blinky...")
    try:
        blink_cycle()
    except KeyboardInterrupt:
        turn_off_all()
        print("Stopped.")
