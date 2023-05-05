#!/bin/bash

echo "Running kfs unit tests"

#	Copy test binary to grub dir
cp -f $1 iso/boot/$2

# Changing grub timeout
if [ "$(uname)" == "Darwin" ]; then
	sed -i '' "s/timeout=./timeout=0/" iso/boot/grub/grub.cfg
else
	sed -i "s/timeout=./timeout=0/" iso/boot/grub/grub.cfg
fi

#	Build iso file using test binary and grub
grub-mkrescue -o $2 iso

DIR_LOGS=logs
mkdir -p $DIR_LOGS

QEMU_DEBUG_ARGS=''
if [ "$4" == "debug" ]; then
	QEMU_DEBUG_ARGS="$QEMU_DEBUG_ARGS -s -S"
fi
# Run qemu for kernel unit tests
if [ "$3" == "test" ]; then 
	qemu-system-i386 $QEMU_DEBUG_ARGS\
                   	 -d int \
				     -drive format=raw,file=$2 \
                     -chardev stdio,id=char0,logfile=$DIR_LOGS/kernel.log\
					 -serial chardev:char0\
				     -serial file:$DIR_LOGS/debug_kernel.log\
				     -display none \
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
# Run qemu for kernel run
else
	qemu-system-i386 $QEMU_ARGS\
		             -soundhw pcspk\
				     -rtc base=localtime\
				     -no-reboot\
				     -d int\
				     -drive format=raw,file=$2\
				     -serial file:$DIR_LOGS/kernel.log\
				     -serial file:$DIR_LOGS/debug_kernel.log\
				     -device isa-debug-exit,iobase=0xf4,iosize=0x04\
				     2> $DIR_LOGS/qemu.log
fi
