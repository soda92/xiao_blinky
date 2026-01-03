MEMORY
{
  /* NRF52840 with Softdevice S140 7.3.0 */
  /* Flash: 156KB reserved for SoftDevice */
  FLASH (rx) : ORIGIN = 0x27000, LENGTH = 0xED000 - 0x27000

  /* RAM: 31KB reserved for SoftDevice (safe default for central+peripheral) */
  RAM (rwx)  : ORIGIN = 0x20007C00, LENGTH = 0x20040000 - 0x20007C00
}
