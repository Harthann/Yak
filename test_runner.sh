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

echo "" > kernel.log

qemu-system-i386 -d int \
			-drive format=raw,file=$2 \
			-nographic \
			-device isa-debug-exit,iobase=0xf4,iosize=0x04 2> qemu.log | less +F | awk '
  /ok/ {len=split($0,a," "); a[len] = "\033[32m" a[len] "\033[39m"; for (i=0; i<=len; i++){printf "%s ",a[i]}; printf "\n"; system("")}'

ret=${PIPESTATUS[0]}

if [ $ret -eq 33 ]; then
	exit 0;
fi
exit 1;
