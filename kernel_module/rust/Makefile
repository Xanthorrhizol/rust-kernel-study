# SPDX-License-Identifier: GPL-2.0
ccflags-y += -I$(src)				# needed for trace events

rust-m += rust_printk.rs

obj-m += rust_printk.o
obj-m += module_param_c.o

all:
	make -C ${HOME}/study/rust-kernel/linux M=$(PWD) modules
clean:
	make -C ${HOME}/study/rust-kernel/linux M=$(PWD) clean
