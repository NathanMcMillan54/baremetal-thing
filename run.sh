#!/bin/bash

sh build.sh
echo "Running thing.img in Qemu..."
echo ""
qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel thing.img
