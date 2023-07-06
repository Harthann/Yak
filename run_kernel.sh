#!/bin/bash

# Argument expect
# 1: Kernel binary
# 2: Destination for grub directory
# 3: Mode test/boot
# 4: QEMU additional arguments like -s -S in case of debug

#Copy test binary to grub dir
cp -f $1 iso/boot/$2

# Set timeout value to 0 for test or 5 for normal boot
[ "$3" == "test" ] && timeout=0 || timeout=5

# Changing grub timeout (sed command is different between Mac and Linux)
[ "$(uname)" == "Darwin" ] && sed -i '' 's/timeout=.*/timeout='$timeout'/' iso/boot/grub/grub.cfg \
                           || sed -i    's/timeout=.*/timeout='$timeout'/' iso/boot/grub/grub.cfg

#	Build iso file using test binary and grub
#	Compress the binary if it is located in rust release target directory
[ "$(basename $(dirname $1))" == "release" ] && grub-mkrescue --compress=xz -o $2 iso \
                                             || grub-mkrescue -o $2 iso

# Create log directory
DIR_LOGS=logs
mkdir -p $DIR_LOGS

# Sound hardware depends on Host OS
[ $(uname) == "Darwin" ] && AUDIODEV="coreaudio" || AUDIODEV="pa"

# Expect qemu args to be in arg 4
# In case of test, redirect logfile output to stdio as well
[ "$3" == "test" ] && QEMU_ARGS="-chardev stdio,id=char0,logfile=$DIR_LOGS/kernel.log -display none" \
                   || QEMU_ARGS="-chardev file,id=char0,path=$DIR_LOGS/kernel.log -display curses"
QEMU_ARGS="
$QEMU_ARGS
$4
-audiodev $AUDIODEV,id=audio0 -machine pcspk-audiodev=audio0
-d int
-drive id=disk,file=$2,format=raw,if=none
-device ide-hd,drive=disk,bus=ide.0
-serial chardev:char0
-serial file:$DIR_LOGS/debug_kernel.log
-no-reboot
-rtc base=localtime
-device isa-debug-exit,iobase=0xf4,iosize=0x04
"

# Run qemu for kernel unit tests
if [ "$3" == "test" ]; then 
	qemu-system-i386 $QEMU_ARGS 2> logs/qemu.log | awk "
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
	qemu-system-i386 $QEMU_ARGS 2> $DIR_LOGS/qemu.log
fi

