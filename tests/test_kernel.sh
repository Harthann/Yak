#!/bin/bash

echo "Running kfs unit tests"

#	Copy test binary to grub dir
cp $1 iso/boot/$2

# Changing grub timeout
if [ "$(uname)" == "Darwin" ]; then
	sed -i '' "s/timeout=./timeout=0/" iso/boot/grub/grub.cfg
else
	sed -i "s/timeout=./timeout=0/" iso/boot/grub/grub.cfg
fi

#	Build iso file using test binary and grub
grub-mkrescue -o $2 iso

echo "" > logs/kernel.log

qemu-system-i386 -d int \
			-drive format=raw,file=$2 \
			-nographic \
			-no-reboot \
			-device isa-debug-exit,iobase=0xf4,iosize=0x04 2> logs/qemu.log | awk "
  /[ok]/ {sub(/\[ok\]/,\"[\033[32mok\033[39m]\");}
  /[failed]/ {sub(/\[failed\]/,\"[\033[31mfailed\033[39m]\");}
  // {print; system(\"\")}"

ret=${PIPESTATUS[0]}

if [ $ret -eq 33 ]; then
	exit 0;
fi
exit 1;
