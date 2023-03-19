LD = ld
LDFLAGS = -static -Bsymbolic -nostdlib -z noexecstack
ASM = x86_64-elf-as
ASMFLAGS = -g

BINDIR = bin
OBJDIR = obj
SRCDIR = src

SOURCES = $(shell find src/ -type f -name '*.rs')
ASM_SOURCES = $(shell find src/ -type f -name '*.s')
LIBKERNEL = target/x86_64-unknown-none/debug/libkernel.a

X86_64_HARDWARE_DIR = ../libraries/x86_64_hardware
X86_64_HARDWARE_LIB = $(X86_64_HARDWARE_DIR)/bin/x86_64_hardware.a

OBJS = $(patsubst $(SRCDIR)/%.s, $(OBJDIR)/%.o, $(ASM_SOURCES))
OBJS += $(LIBKERNEL)
OBJS += $(X86_64_HARDWARE_LIB)

KERNEL = $(BINDIR)/kernel.elf

LINKER_SCRIPT = kernel.ld

all: $(KERNEL)

$(OBJDIR)/%.o: $(SRCDIR)/%.s
	@ mkdir -p $(@D)
	$(ASM) $(ASMFLAGS) -c $^ -o $@

$(LIBKERNEL): $(SOURCES)
	cargo build --target x86_64-unknown-none

$(BINDIR):
	@mkdir -p $(BINDIR)

$(KERNEL): $(BINDIR) $(OBJS) $(LINKER_SCRIPT)
	$(LD) -T $(LINKER_SCRIPT) $(LDFLAGS) -o $(KERNEL) $(OBJS)

$(X86_64_HARDWARE_LIB):
	cd $(X86_64_HARDWARE_DIR) && make all

.PHONY: buildimg all clean $(LIBKERNEL) $(KERNEL)

clean:
	-rm -rf $(BINDIR)
	-rm -rf $(OBJDIR)
	cargo clean