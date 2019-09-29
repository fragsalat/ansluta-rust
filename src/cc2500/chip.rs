use super::constants::*;
use super::COMMAND;
use super::STATE;
use super::Address;
use rppal::gpio::Gpio;
use rppal::spi::{Polarity};
use rppal::spi::{Segment, Spi};
use std::thread;

pub struct CC2500 {
    pub spi: Spi,
    pub address: Option<Address>,
}

/**
 * Block execution until MISO is set to high and chip is ready for receiving data
 */
fn wait_for_miso() {
    let gpio = Gpio::new().expect("Couldn't iunit GPIO");
    let miso = gpio
        .get(9)
        .expect("Could not attach to MISO pin")
        .into_output();

    while miso.is_set_high() {}
}

/**
 * Create list of segments for the list of bytes
 */
fn bytes_to_segments(bytes: &[u8], delay: u16) -> Vec<Segment> {
    let segments = bytes.iter().map(|byte| {
        let mut segment = Segment::with_write(std::slice::from_ref(byte));
        segment.set_delay(delay);
        segment
    });
    return segments.collect();
}

impl CC2500 {

    /**
     * Write spi register on CC2500 chip
     */
    fn write_reg(&mut self, reg: u8, value: u8) {
        self.spi.set_ss_polarity(Polarity::ActiveLow).expect("Couldn't set polarity");
        wait_for_miso();

        let bytes = [reg, value];
        let segments = bytes_to_segments(&bytes, 200);
        self.spi
            .transfer_segments(&segments)
            .expect("Could not write");

        self.spi.set_ss_polarity(Polarity::ActiveHigh).expect("Couldn't set polarity");

        thread::sleep(DELAY_200);
    }

    /**
     * Read spi register from CC2500 chip
     */
    fn read_reg(&mut self, addr: u8) -> u8 {
        self.spi.set_ss_polarity(Polarity::ActiveLow).expect("Couldn't set polarity");
        wait_for_miso();

        let buffer = [addr + 0x80];
        let mut write = Segment::with_write(&buffer);
        write.set_delay(200);

        let mut buffer: [u8; 1] = [1];
        let mut read = Segment::with_read(&mut buffer);
        read.set_delay(200);

        self.spi
            .transfer_segments(&[write, read])
            .expect("Could not write");

        self.spi.set_ss_polarity(Polarity::ActiveHigh).expect("Couldn't set polarity");

        return buffer[0];
    }

    /**
     * Send a strobe of 1 byte to the CC2500
     */
    pub fn strobe(&mut self, state: STATE) {
        self.spi.set_ss_polarity(Polarity::ActiveLow).expect("Couldn't set polarity");
        wait_for_miso();

        self.spi
            .write(&[state as u8])
            .expect("Could not transfer to SPI");

        self.spi.set_ss_polarity(Polarity::ActiveHigh).expect("Couldn't set polarity");

        thread::sleep(DELAY_20000);
    }

    /**
     * Initialize the registers of CC2500 chip
     */
    pub fn init(&mut self) {
        self.strobe(STATE::SRES);

        self.write_reg(REG_IOCFG2, VAL_IOCFG2);
        self.write_reg(REG_IOCFG0, VAL_IOCFG0);
        self.write_reg(REG_PKTLEN, VAL_PKTLEN);
        self.write_reg(REG_PKTCTRL1, VAL_PKTCTRL1);
        self.write_reg(REG_PKTCTRL0, VAL_PKTCTRL0);
        self.write_reg(REG_ADDR, VAL_ADDR);
        self.write_reg(REG_CHANNR, VAL_CHANNR);
        self.write_reg(REG_FSCTRL1, VAL_FSCTRL1);
        self.write_reg(REG_FSCTRL0, VAL_FSCTRL0);
        self.write_reg(REG_FREQ2, VAL_FREQ2);
        self.write_reg(REG_FREQ1, VAL_FREQ1);
        self.write_reg(REG_FREQ0, VAL_FREQ0);
        self.write_reg(REG_MDMCFG4, VAL_MDMCFG4);
        self.write_reg(REG_MDMCFG3, VAL_MDMCFG3);
        self.write_reg(REG_MDMCFG2, VAL_MDMCFG2);
        self.write_reg(REG_MDMCFG1, VAL_MDMCFG1);
        self.write_reg(REG_MDMCFG0, VAL_MDMCFG0);
        self.write_reg(REG_DEVIATN, VAL_DEVIATN);
        self.write_reg(REG_MCSM2, VAL_MCSM2);
        self.write_reg(REG_MCSM1, VAL_MCSM1);
        self.write_reg(REG_MCSM0, VAL_MCSM0);
        self.write_reg(REG_FOCCFG, VAL_FOCCFG);
        self.write_reg(REG_BSCFG, VAL_BSCFG);
        self.write_reg(REG_AGCCTRL2, VAL_AGCCTRL2);
        self.write_reg(REG_AGCCTRL1, VAL_AGCCTRL1);
        self.write_reg(REG_AGCCTRL0, VAL_AGCCTRL0);
        self.write_reg(REG_WOREVT1, VAL_WOREVT1);
        self.write_reg(REG_WOREVT0, VAL_WOREVT0);
        self.write_reg(REG_WORCTRL, VAL_WORCTRL);
        self.write_reg(REG_FREND1, VAL_FREND1);
        self.write_reg(REG_FREND0, VAL_FREND0);
        self.write_reg(REG_FSCAL3, VAL_FSCAL3);
        self.write_reg(REG_FSCAL2, VAL_FSCAL2);
        self.write_reg(REG_FSCAL1, VAL_FSCAL1);
        self.write_reg(REG_FSCAL0, VAL_FSCAL0);
        self.write_reg(REG_RCCTRL1, VAL_RCCTRL1);
        self.write_reg(REG_RCCTRL0, VAL_RCCTRL0);
        self.write_reg(REG_FSTEST, VAL_FSTEST);
        self.write_reg(REG_TEST2, VAL_TEST2);
        self.write_reg(REG_TEST1, VAL_TEST1);
        self.write_reg(REG_TEST0, VAL_TEST0);
        self.write_reg(REG_DAFUQ, VAL_DAFUQ);
        // Set power level to max
        self.write_reg(0x3E, 0xFF);
    }

    /**
     * Send command with CC2500 to Ansluta. Repeating the message 50 times to ensure it was consumed
     */
    pub fn command(&mut self, cmd: COMMAND) {
        let cmd_address = cmd as u8;
        match self.address {
            None => panic!("Can not send command to unknown address!"),
            Some(address) => {
                for _i in 0..50 {
                    self.strobe(STATE::SIDLE);
                    self.strobe(STATE::SFTX);

                    self.spi.set_ss_polarity(Polarity::ActiveLow).expect("Couldn't set polarity");
                    wait_for_miso();

                    let bytes = vec![
                        0x7F,
                        0x06,
                        0x55,
                        0x01,
                        address.0,
                        address.1,
                        cmd_address,
                        0xAA,
                        0xFF,
                    ];

                    let segments: Vec<_> = bytes_to_segments(&bytes, 0);
                    self.spi
                        .transfer_segments(&segments)
                        .expect("Could not write");

                    self.spi.set_ss_polarity(Polarity::ActiveHigh).expect("Couldn't set polarity");

                    self.strobe(STATE::STX);
                    thread::sleep(DELAY_10);
                }
            }
        }
    }

    /**
     * Debugger function to read the address of the original ansulta remote.
     * Once it's read it will print it. In future this should be persisted for other calls
     */
    pub fn read_address(&mut self) -> Option<Address> {
        loop {
            self.strobe(STATE::SRX);
            self.write_reg(REG_IOCFG1, 0x01); // Switch MISO to output if packet has been received
            thread::sleep(DELAY_200);

            let packet_length = self.read_reg(REG_FIFO) as usize;
            let mut packet = vec![0; packet_length as usize];

            if packet_length > 8 {
                println!("Received packet is to big {}", packet_length);
            }
            // Read packet from FIFO buffer
            if packet_length > 0 && packet_length <= 8 {
                println!("Received packet");

                for i in 0..packet_length {
                    packet[i] = self.read_reg(REG_FIFO);
                }
                println!("Packet {:x?}", packet);

                let start = packet.iter().position(|&byte| byte != 0x55).unwrap();

                // Check if the packet is from IKEA remote
                if packet[start] == 0x01 && packet[start + 4] == 0xAA {
                    println!("Address found: {} {}", packet[start + 1], packet[start + 2]);
                    self.address = Some(Address(packet[start + 1], packet[start + 2]));

                    self.strobe(STATE::SIDLE);
                    self.strobe(STATE::SFRX);

                    return self.address.clone();
                }
            }

            self.strobe(STATE::SIDLE);
            self.strobe(STATE::SFRX);
        }
    }

    /**
     * Set address of original ansluta
     */
    pub fn set_address(&mut self, address: Address) {
        self.address = Some(address.clone());
    }
}
