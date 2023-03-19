ASM = x86_64-elf-as
ASMFLAGS = -g

BINDIR = bin
OBJDIR = obj
SRCDIR = src

ASM_SOURCES = $(shell find src/ -type f -name '*.s')
OBJS = $(patsubst $(SRCDIR)/%.s, $(OBJDIR)/%.o, $(ASM_SOURCES))

X86_64_HARDWARE_LIB = $(BINDIR)/x86_64_hardware.a

all: $(X86_64_HARDWARE_LIB)

$(X86_64_HARDWARE_LIB): $(OBJS)
	@ mkdir -p $(@D)
	ar rcs $@ $^

$(OBJDIR)/%.o: $(SRCDIR)/%.s
	@ mkdir -p $(@D)
	$(ASM) $(ASMFLAGS) -c $^ -o $@

clean:
	-rm -rf $(BINDIR)
	-rm -rf $(OBJDIR)
	cargo clean


.PHONY : clean all
