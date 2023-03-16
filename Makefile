BOOTLOADER_DIR = bootloader_uefi
ASSETS_DIR = assets

all: modules

modules:
	cd $(BOOTLOADER_DIR) && make all
	cd $(ASSETS_DIR) && make all

clean:
	cd $(BOOTLOADER_DIR) && make clean

clean-all: clean
	cd $(ASSETS_DIR) && make clean

.PHONY: all modules clean clean-all