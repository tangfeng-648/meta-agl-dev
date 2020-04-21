obj-m +=  edac_inject.o

export KDIR := /lib/modules/$(shell uname -r)/build

allofit:  modules
modules:
	@$(MAKE) -C $(KDIR) M=$(shell pwd) modules
kernel_clean:
	@$(MAKE) -C $(KDIR) M=$(shell pwd) clean

clean: kernel_clean
	rm -rf   Module.symvers modules.order
