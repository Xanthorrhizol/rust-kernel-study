#!/bin/bash
cd $(dirname "$(readlink -f "$0")")
if [ ! -d busybox ]; then
  git clone https://git.busybox.net/busybox
  cd busybox
else
  cd busybox
  make distclean
fi
echo "Enable static build"
read -p "Press [Enter] to continue..."
make menuconfig
make install CONFIG_PATH=../busybox_install_result
cd ..
mkdir initramfs
cd initramfs
mkdir -p bin sbin etc proc sys usr/bin usr/sbin
cp -a ../busybox_install_result/* .
cat > init << EOF
#!/bin/sh
/bin/mount -t devtmpfs devtmpfs /dev
/bin/mount -t proc none /proc
/bin/mount -t sysfs none /sys
exec 0</dev/console
exec 1>/dev/console
exec 2>/dev/console
exec /bin/sh
EOF
chmod +x init
cat > mount_rust_modules.sh << EOF
#!/bin/sh
if [ ! -d /mnt ]; then
  mkdir /mnt
fi
mount -t 9p -o trans=virtio rust_modules /mnt
EOF
command -v cpio >/dev/null
if [ $? -ne 0 ]; then
  sudo pacman -S cpio
fi
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
