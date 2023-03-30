# Virtual Memory Manager Design

The _Virtual Memory Manager_ (VMM) will be used to manage the creation and lifetime of virtual
memory spaces. Note the actual strategies for swap and physical frame selection will be outside the
kernel. The kernel will only handle setting up mappings as demanded and will deal with a page fault
by suspending the faulted process and forwarding the details onto the user space memory manager.

The following terms will be used by this document:
- VMemN - The Nth virtual memory space managed by the kernel.
- VMem0 - The kernels own memory space that will be mapped into every subsequent VMemN
- Wired - The act of making a memory space ineligible for swapping out. Wired spaces are always in
memory. This is typically used for the kernel itself and for key device drivers and services

## Requirements

### Functional Requirements

#### FR1 - VMem0 must operate independently of the kernel heap

I.E. any memory the kernel needs it must allocate itself using the page frame allocator. If VMem0
triggers a VMem0 sbrk then it is likely the kernel will lock up.

#### FR2 - VMem0 must maintain its own independent locking

If expanding the table of VMemN requires a heap expansion in VMem0 it should be possible to do this
while the VMemN table is still locked.

#### FR3 - Wired heap expansion should automatically allocate frames

Moving the break point for a wired memory space should involve premapping the memory. The kernel,
the core user space memory manager and the paging server all should never page fault given they are
necessary to service page faults.

#### FR4 - Non-Wired heap expansion should allocate on a page fault

When a process expands a non-wired heap the first access should trigger the allocation process. It
should not be done pre-emptively