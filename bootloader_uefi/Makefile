SOURCES = $(shell find src/ -type f -name '*.rs')

BOOTLOADER = target/x86_64-unknown-uefi/debug/bootloader_uefi.efi

all: $(BOOTLOADER)

$(BOOTLOADER): $(SOURCES)
	cargo build --target x86_64-unknown-uefi

clean:
	cargo clean

.DEFAULT_GOAL := all
.PHONY : clean all
