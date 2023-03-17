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
OBJS = $(patsubst $(SRCDIR)/%.s, $(OBJDIR)/%.o, $(ASM_SOURCES))
OBJS += $(LIBKERNEL)

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

.PHONY: buildimg all clean $(LIBKERNEL) $(KERNEL)

clean:
	-rm -rf $(BINDIR)
	-rm -rf $(OBJDIR)
	cargo clean