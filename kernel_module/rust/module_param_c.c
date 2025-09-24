// SPDX-License-Identifier: GPL-2.0
#include <linux/init.h>
#include <linux/module.h>
#include <linux/moduleparam.h>
#include <linux/types.h>

bool my_bool;
int my_int;
unsigned long my_usize;
char my_str[14];
int my_array[3];

module_param(my_bool, bool, 0);
module_param(my_int, int, 0644);
module_param_string(my_str, my_str, sizeof(my_str), 0644);
module_param(my_usize, ulong, 0644);
module_param_array(my_array, int, NULL, 0);

MODULE_LICENSE("GPL");
MODULE_DESCRIPTION(
    "Module Parameter Shim Example; I gave up to write this module with rust");
MODULE_AUTHOR("Xanthorrhizol");
MODULE_VERSION("0.1");

static int __init module_param_init(void) {
  printk(KERN_INFO "Module parameters:\n");
  printk(KERN_INFO "  my_bool: %d\n", my_bool);
  printk(KERN_INFO "  my_int: %d\n", my_int);
  printk(KERN_INFO "  my_str: %s\n", my_str);
  printk(KERN_INFO "  my_usize: %lu\n", my_usize);
  printk(KERN_INFO "  my_array: [%d, %d, %d]\n", my_array[0], my_array[1],
         my_array[2]);
  return 0;
}

static void __exit module_param_exit(void) {
  printk(KERN_INFO "Module exiting.\n");
}

module_init(module_param_init);
module_exit(module_param_exit);
