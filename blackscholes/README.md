# TradeShark

A Rust project cross-compiled and deployed to a Terasic DE10-Standard FPGA SoC.

## Hardware

- **Board:** Terasic DE10-Standard (Intel Cyclone V SoC)
- **OS:** Ubuntu 16.04 LTS (running on the ARM HPS)
- **Connection:** Ethernet + UART serial

## Project Structure

```
blackscholes/
├── src/
│   └── main.rs        # Main implementation
├── .cargo/
│   └── config.toml    # Cross-compilation config
├── Cargo.toml
├── deploy.sh          # Build and deploy script
└── README.md
```

## Prerequisites

- Rust + Cargo (via [rustup](https://rustup.rs))
- WSL2 (Windows) or Linux/macOS
- ARM musl cross-compilation target and linker:

```bash
rustup target add armv7-unknown-linux-musleabihf
sudo apt install gcc-arm-linux-gnueabihf
```

## Building and Deploying

Update `BOARD_IP` in `deploy.sh` to match your board's IP address, then:

```bash
chmod +x deploy.sh
./deploy.sh
```

This will cross-compile a statically linked ARM binary and copy it to `~/TradeShark/` on the board via SCP.

## Running on the Board

SSH into the board:

```bash
ssh root@<board-ip>
```

Then run:

```bash
cd ~/TradeShark
./blackscholes
```

## Why Static Linking?

The board runs Ubuntu 16.04 which has an old glibc. Binaries are compiled against `armv7-unknown-linux-musleabihf` (musl libc) so the binary is fully self-contained and doesn't depend on the board's system libraries.