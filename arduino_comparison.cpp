#include <bluefruit.h>

// BLE Service and Characteristics
BLEUart bleuart;

// LED Pins (Xiao nRF52840 Sense)
// Note: Arduino defines these constants for you
const int PIN_RED   = LED_RED;
const int PIN_GREEN = LED_GREEN;
const int PIN_BLUE  = LED_BLUE;

void setup() {
  // 1. Setup GPIO
  pinMode(PIN_RED, OUTPUT);
  pinMode(PIN_GREEN, OUTPUT);
  pinMode(PIN_BLUE, OUTPUT);
  
  // Turn all off (High = OFF for common anode)
  digitalWrite(PIN_RED, HIGH);
  digitalWrite(PIN_GREEN, HIGH);
  digitalWrite(PIN_BLUE, HIGH);

  // 2. Setup BLE
  Bluefruit.begin();
  Bluefruit.setTxPower(4);    // Check bluefruit.h for supported values
  Bluefruit.setName("XiaoBLE");

  // Configure and Start BLE Uart Service
  bleuart.begin();

  // Set up and start advertising
  startAdv();
}

void startAdv(void) {
  // Advertising packet
  Bluefruit.Advertising.addFlags(BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE);
  Bluefruit.Advertising.addTxPower();
  
  // Include bleuart 128-bit UUID
  Bluefruit.Advertising.addService(bleuart);

  // Secondary Scan Response packet (Optional)
  // There is no room for 'Name' in Advertising packet
  Bluefruit.ScanResponse.addName();
  
  /* Start Advertising
   * - Enable auto advertising if disconnected
   * - Interval:  fast mode = 20 ms, slow mode = 152.5 ms
   * - Timeout for fast mode is 30 seconds
   * - Start(timeout) with timeout = 0 will advertise forever (until connected)
   * 
   * For recommended advertising interval
   * https://developer.apple.com/library/content/qa/qa1931/_index.html   
   */
  Bluefruit.Advertising.restartOnDisconnect(true);
  Bluefruit.Advertising.setInterval(32, 244);    // in unit of 0.625 ms
  Bluefruit.Advertising.setFastTimeout(30);      // number of seconds in fast mode
  Bluefruit.Advertising.start(0);                // 0 = Don't stop advertising after n seconds  
}

// 3. Main Loop
// In Arduino, we simulate multitasking by checking time
unsigned long previousMillis = 0;
const long interval = 500;
int step = 0; // 0=Red, 1=Green, 2=Blue

void loop() {
  unsigned long currentMillis = millis();

  // Non-blocking blink logic
  if (currentMillis - previousMillis >= interval) {
    previousMillis = currentMillis;

    // Reset all LEDs
    digitalWrite(PIN_RED, HIGH);
    digitalWrite(PIN_GREEN, HIGH);
    digitalWrite(PIN_BLUE, HIGH);

    if (step == 0) {
      digitalWrite(PIN_RED, LOW);
      if (Bluefruit.connected()) {
        bleuart.print("Red LED On\n");
      }
    } else if (step == 1) {
      digitalWrite(PIN_GREEN, LOW);
      if (Bluefruit.connected()) {
        bleuart.print("Green LED On\n");
      }
    } else if (step == 2) {
      digitalWrite(PIN_BLUE, LOW);
      if (Bluefruit.connected()) {
        bleuart.print("Blue LED On\n");
      }
    }

    step++;
    if (step > 2) step = 0;
  }

  // Handle incoming data (Echo it back?)
  // Note: Rust version printed it to RTT log
  while (bleuart.available()) {
    uint8_t ch = (uint8_t) bleuart.read();
    bleuart.write(ch); // Echo back
  }
}
