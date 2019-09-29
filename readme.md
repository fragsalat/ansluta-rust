# Ansluta Rust controller

This project is a rust application compiled for an raspberry pi to control IKEA lamps by faking an Ansluta remote.
Thanks to [NDBCK](https://github.com/NDBCK/Ansluta-Remote-Controller) for his great research and example code in C++.
In this project I used the first time ever rust language so please don't judge about code style :D.


### Requirements

* [Raspberry PI 3 <small>(€33,99 at Amazon)</small>](https://amzn.to/2oiGepb)
* [CC2500 2.4 GHz chip <small>(€3 at Amazon)</small>](https://amzn.to/2mbaRwk)
* [Breadboard & Jumper cables <small>(€8.99 at Amazon)</small>](https://amzn.to/2mRyfiG)
* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo make](https://github.com/sagiegurari/cargo-make#installation)
* [Cargo watch](https://github.com/passcod/cargo-watch#install)
* Linux Subsystem if you are on windows
* GCC for Raspberry ARM processor

### Setup

**1)** Open linux shell (`bash.exe` for linux subsystem on windows)

**2)** Install rustup, rust and cargo 
```bash
$ curl https://sh.rustup.rs -sSf | sh
```

**3)** Install cargo-make
```
$ cargo install --force cargo-make
```

**4)** Install cargo-make
```
$ cargo install --force cargo-watch
```

**5)** Install ARM gcc to cross compile for raspberry pi. 
*Note: If you choose another gcc version you have to update the linker used in the Cargo.toml file.*
```
$ apt-get install gcc-8-multilib-arm-linux-gnueabihf
```

**6)** Add target in rustup 
```
$ rustup target add armv7-unknown-linux-gnueabihf
```

**7)** Update ip in Cargo.toml to directly upload to raspberry pi on build
```
$ sed -i 's/xxx.xxx.xxx.xxx/your-ip/g' ./Cargo.toml
```

### Run

**Build**: Compile and upload to raspberry pi
```bash
$ cargo make --makefile Cargo.toml debug-raspberry
```

**Watch**: Compile and upload on every file change
```bash
cargo watch -x "make --makefile Cargo.toml debug-raspberry"
```

**NOTE**: Seems like there is no remote debug functionality at all. I didn't managed it to have breakpoints working. Neither CLion with GDBRemote nor CLI worked and I just found bugs / issues raised by other users complaining about the same.