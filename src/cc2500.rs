use crate::cc2500::chip::CC2500;
use rppal::spi::Spi;

mod chip;
mod constants;

#[derive(Debug, Copy, Clone)]
pub struct Address(pub u8, pub u8);

pub enum STATE {
    SIDLE = 0x36, // Exit RX / TX
    STX = 0x35,   // Enable TX. If in RX state, only enable TX if CCA passes
    SFTX = 0x3B,  // Flush the TX FIFO buffer. Only issue SFTX in IDLE or TXFIFO_UNDERFLOW states
    SRES = 0x30,  // Reset chip
    SRX = 0x34,   // Enable RX. Perform calibration if enabled
    SFRX = 0x3A,  // Flush the RX FIFO buffer. Only issue SFRX in IDLE or RXFIFO_OVERFLOW states
}

pub enum COMMAND {
    LightOff = 0x01,   // Command to turn the light off
    LightOn50 = 0x02,  // Command to turn the light on 50%
    LightOn100 = 0x03, // Command to turn the light on 100%
    PAIR = 0xFF,       // Command to pair a remote to the light
}

pub fn new(spi: Spi) -> CC2500 {
    CC2500 { spi, address: None }
}
