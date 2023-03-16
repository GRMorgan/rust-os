
BOOTLOADER_DIR = bootloader_uefi

all: modules

modules:
	cd $(BOOTLOADER_DIR) && make all

clean:
	cd $(BOOTLOADER_DIR) && make clean

.PHONY: all modules