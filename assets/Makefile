OVFM_CODE_FILENAME = OVMF_CODE-pure-efi.fd
OVFM_CODE_WEB_ADDRESS = https://github.com/rust-osdev/ovmf-prebuilt/releases/download/v0.20220719.209%2Bgf0064ac3af/OVMF_CODE-pure-efi.fd

OVFM_VARS_FILENAME = OVMF_VARS-pure-efi.fd
OVFM_VARS_WEB_ADDRESS = https://github.com/rust-osdev/ovmf-prebuilt/releases/download/v0.20220719.209%2Bgf0064ac3af/OVMF_VARS-pure-efi.fd

ALL_ASSETS = $(OVFM_CODE_FILENAME) $(OVFM_VARS_FILENAME)

$(OVFM_CODE_FILENAME):
	wget $(OVFM_CODE_WEB_ADDRESS) -O $@

$(OVFM_VARS_FILENAME):
	wget $(OVFM_VARS_WEB_ADDRESS) -O $@

all: $(ALL_ASSETS)

clean:
	-rm $(ALL_ASSETS)

.DEFAULT_GOAL := all
.PHONY : all clean
