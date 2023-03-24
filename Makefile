BOOTLOADER_DIR = bootloader_uefi
ASSETS_DIR = assets
KERNEL_DIR = kernel
X86_64_HARDWARE_DIR = libraries/x86_64_hardware
BINDIR = bin
OSNAME = RustOs
OSIMAGE = $(BINDIR)/$(OSNAME).img
BOOTLOADER = $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/debug/bootloader_uefi.efi
KERNEL = $(KERNEL_DIR)/bin/kernel.elf

all: $(OSIMAGE)

$(OSIMAGE): modules
	@mkdir -p $(@D)
	dd if=/dev/zero of=$(OSIMAGE) bs=512 count=93750
	mformat -i $(OSIMAGE) -i 1440 ::
	mmd -i $(OSIMAGE) ::/EFI
	mmd -i $(OSIMAGE) ::/EFI/BOOT
	mcopy -i $(OSIMAGE) $(BOOTLOADER) ::/EFI/BOOT/BOOTX64.EFI
	mcopy -i $(OSIMAGE) $(KERNEL) ::

modules:
	cd $(BOOTLOADER_DIR) && make all
	cd $(KERNEL_DIR) && make all
	cd $(ASSETS_DIR) && make all

clean:
	cd $(BOOTLOADER_DIR) && make clean
	cd $(KERNEL_DIR) && make clean
	cd $(X86_64_HARDWARE_DIR) && make clean
	-rm -rf $(BINDIR)

clean-all: clean
	cd $(ASSETS_DIR) && make clean

run: $(OSIMAGE)
	qemu-system-x86_64 -drive file="$(OSIMAGE)",format=raw -m 256M -cpu qemu64 -drive if=pflash,format=raw,unit=0,file="assets/OVMF_CODE-pure-efi.fd",readonly=on -drive if=pflash,format=raw,unit=1,file="assets/OVMF_VARS-pure-efi.fd" -net none -serial stdio > out.txt

run-debug: $(OSIMAGE)
	qemu-system-x86_64 -s -S -hda $(OSIMAGE) -m 256M -cpu qemu64 -drive if=pflash,format=raw,unit=0,file="assets/OVMF_CODE-pure-efi.fd",readonly=on -drive if=pflash,format=raw,unit=1,file="assets/OVMF_VARS-pure-efi.fd" -net none -serial stdio > out.txt

test:
	cd libraries && make test


.PHONY: all modules clean clean-all run run-debug test