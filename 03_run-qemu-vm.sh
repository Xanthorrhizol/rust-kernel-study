#!/bin/bash
qemu-system-x86_64 -kernel linux/arch/x86_64/boot/bzImage -nographic -append "console=ttyS0,115200 keep_bootcon root=/dev/ram0 rdinit=/init" -initrd ./initramfs.cpio.gz -virtfs local,path=./kernel_module,security_model=none,mount_tag=rust_modules
