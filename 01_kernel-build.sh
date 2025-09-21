#!/bin/bash
cd $(dirname "$(readlink -f "$0")")
if [ ! -d linux ]; then
  git clone https://github.com/Rust-for-Linux/linux.git
  cd linux
else
  cd linux
  make distclean
fi
make rustavailable
if [ $? -ne 0 ]; then
  sudo pacman -S rust-bindgen
fi
echo "1. Enable Rust support in the kernel configuration"
echo "2. Enable all as 'M' Kernel hacking - Sample kernel code -> Rust samples"
echo "  (!) Host programs: '*'"
echo "3. Enable Kernel hacking - printk and dmesg options - Enable dynamic printk() support"
read -p "Press [Enter] to continue..."
make menuconfig
make
cp samples/rust/*.ko ../kernel_module/
