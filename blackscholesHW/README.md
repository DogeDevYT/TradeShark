# TradeShark

A Rust project cross-compiled and deployed to a Terasic DE10-Standard FPGA SoC.

## Hardware

- **Board:** Terasic DE10-Standard (Intel Cyclone V SoC)
- **OS:** Ubuntu 16.04 LTS (running on the ARM HPS)
- **Connection:** Ethernet + UART serial

## Project Structure

```
blackscholesHW/
├── coprocessor/       # HDL for Coprocessor 
├── full/              # HDL for full implementation
├── ip_cores/          # Altera FP Ipcores
├── test_individal.sh  # Run individual tests
└── README.md
```

## Prerequisites

- Quartus IDE
- WSL2 (Windows) or Linux/macOS
- ModelSim


## Quartus Setup
1. Open to Quartus, select new project
2. Select Repository
3. Select 5CSXFC6D6F31C6N as Board model 
4. Add all files from full/coprocessor

## IP Core Setup
1. From IP catalog select FP_Functions
2. Select Verilog option
3. Select variation name fp_{add,divide,exp,sqrt,log,multi,sub}
4. Select correction location ip_core_fp_{add,divide,exp,sqrt,log,multi,sub}
5. For each core, select Double, Enable port
