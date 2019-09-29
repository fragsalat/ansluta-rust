use crate::cc2500::{COMMAND, Address};
use rppal::spi::{BitOrder, Bus, Mode, SlaveSelect, Spi};
use std::env;
use std::fs;

mod cc2500;

const ADDRESS_FILE: &str = "./.ansluta-address";

/**
 * Load 2 byte address of original ansluta
 */
fn load_address() -> Option<Address> {
    match fs::read(ADDRESS_FILE) {
        Ok(address) => Some(Address(address[0], address[1])),
        Err(_) => None
    }
}

/**
 * Save 2 byte address of original ansluta for later use
 */
fn save_address(address: Address) {
    if let Err(error) = fs::write(ADDRESS_FILE, [address.0, address.1]) {
        panic!("Couldn't save ansluta address to {} because {}", ADDRESS_FILE, error);
    }
}

/**
 * Initialize CC2500 chip registers, ensure we have a ansulta address and execute one of the commands off, 50 or 100
 */
fn main() {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_000_000, Mode::Mode0)
        .expect("Couldn't start SPI connection");
    spi.set_bit_order(BitOrder::MsbFirst)
        .expect("Could not set bit order on SPI");

    let mut cc2500 = cc2500::new(spi);
    cc2500.init();
    // Read address from original ansluta if none is known yet
    match load_address() {
        Some(address) => cc2500.set_address(address),
        None => {
            match cc2500.read_address() {
                Some(address) => save_address(address),
                None => panic!("Could not find address!")
            }
        }
    }

    let args: Vec<String> = env::args().collect();
    let level = &args[1];
    let command = {
        match level.as_str() {
            "off" => COMMAND::LightOff,
            "50" => COMMAND::LightOn50,
            "100" => COMMAND::LightOn100,
            _ => panic!("Unknown level. Use off, 50 or 100")
        }
    };

    println!("Light {}", level);
    cc2500.command(command);
}
