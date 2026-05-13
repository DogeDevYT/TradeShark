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

---

## Black-Scholes Call Option Pricing

TradeShark includes a Black-Scholes model for pricing European call options, implemented in the `BS` struct.

### Parameters

| Parameter | Symbol | Description |
|-----------|--------|-------------|
| `security_price` | S | Current price of the underlying security |
| `strike_price` | K | Strike price of the option |
| `risk_free_interest_rate` | r | Risk-free interest rate (annualized) |
| `time_to_maturity` | T - t | Time remaining until expiry (in years) |
| `volatility` | σ | Annualized volatility of the underlying security |

### Formulas

**d₁:**
```
d₁ = [ ln(S/K) + (r + σ²/2)(T-t) ] / [ σ√(T-t) ]
```

**d₂:**
```
d₂ = d₁ - σ√(T-t)
```

**Call option price:**
```
C = S·N(d₁) - K·e^(-r(T-t))·N(d₂)
```

where N(·) is the cumulative distribution function of the standard normal distribution.

### Usage

```rust
use crate::BS;

let bs = BS::new(
    100.0,  // security_price (S)
    105.0,  // strike_price (K)
    0.05,   // risk_free_interest_rate (r)
    1.0,    // time_to_maturity (T - t, in years)
    0.2,    // volatility (σ)
);

let price = bs.call_option_price();
println!("Call option price: {:.4}", price); // ~8.0214
```

### Dependencies

Add the following to `Cargo.toml`:

```toml
[dependencies]
statrs = "0.17"
```

The model uses `statrs::distribution::Normal` for the standard normal CDF.

### Tests

Unit tests covering the call price, d₁, and d₂ calculations can be run with:

```bash
cargo test
```

Expected values for the reference inputs (S=100, K=105, r=0.05, T=1.0, σ=0.2):

| Output | Expected |
|--------|----------|
| d₁ | 0.10609 |
| d₂ | -0.09395 |
| Call price | 8.0214 |