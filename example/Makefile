# project data
export PROJECT_NAME = example

# compiler options
export TARGET = x86_64-unknown-linux-gnu
export COMPILER = gcc-6
export KERNEL_HEADERS ?= /lib/modules/$(shell uname -r)/build
export CARGO = cargo

all:
	$(MAKE) -C $(KERNEL_HEADERS) M=$(CURDIR) CC=$(COMPILER)

clean:
	$(MAKE) -C $(KERNEL_HEADERS) M=$(CURDIR) CC=$(COMPILER) clean
