#!/bin/bash
set -e
 
BOARD_IP="10.0.0.115" # Update with your board's IP address every run!
BOARD_USER="root"
BOARD_DIR="~/TradeShark"
TARGET="armv7-unknown-linux-musleabihf"
BINARY="blackscholes"
 
echo "Building for ARM..."
cargo build --release --target $TARGET
 
echo "Deploying to board..."
ssh $BOARD_USER@$BOARD_IP "mkdir -p $BOARD_DIR"
scp target/$TARGET/release/$BINARY $BOARD_USER@$BOARD_IP:$BOARD_DIR/
 
echo "Done! Binary deployed to $BOARD_DIR/$BINARY"