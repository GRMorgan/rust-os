## Memory Management

### Implement Kernel Heap

This will need to be initialised with enough memory to get the VMM running.
Once the VMM is running we'll call that for the equivalent of brk().

### Implement VMM

This needs to keep track of all the memory spaces in the kernel. Initially it
will only care about Mem0 which is the kernel memory map. It will implement a
full brk/sbrk like interface for the kernel heap but should delegate
fulfilment of page fault handling to the PMM. When a page fault happens it will
send an IPC to the PMM with the details and it will tell it which pages to map

## ACPI Table Mapping

## IDT/Interrupt setup

## PIC initialisation

## PIT initialisation
