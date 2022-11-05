#!/bin/bash

sudo apt-get install -y nasm

mkdir -p $HOME/.local/bin

echo -e '#!/bin/bash\nld -m elf_i386 $@' > $HOME/.local/bin/i386-elf-ld
echo -e '#!/bin/bash\nar $@ --target=elf32-i386' > $HOME/.local/bin/i386-elf-ar

chmod +x $HOME/.local/bin/i386-elf-ar $HOME/.local/bin/i386-elf-ld

curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

sudo apt-get install -y xorriso grub-common mtools qemu-system
