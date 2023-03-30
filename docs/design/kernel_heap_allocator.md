# Kernel Heap Allocator Design

This document is intended to cover the requirements and design of the kernel allocator. It is
expected that this design may evolve over time. Currently the intention is to get something that
works with tolerable average efficiency without going crazy with excessive optimisation

## Assumptions

The following assumptions will be made:

1. We can ignore 32 bit systems entirely
2. The kernel heap will be entirely contiguous
3. A function exists to extend the kernel heap similar to sbrk

## Requirements

### Functional Requirements

#### FR1 - It must support allocation of memory of a given size

The basic function of an allocator. You give it a size and it returns a pointer to a block of
memory large enough to safely used at the given size

#### FR2 - It must support freeing of memory it has previously allocated

It should know how to retrieve and make available memory the kernel has surrendered back to it.

#### FR3 - It must support allocation of memory at various address alignments as defined by POSIX

POSIX defines a _posix\_memalign_ function that provides both a size and an alignment as inputs.
The standard demands that alignment must be a power of 2.

#### FR4 - It must be usable as a Rust **global_allocator**

The purpose of this is to allow Rust constructs like Vec or Box to be used within the kernel

## Design

Largely we are going for a design based upon dlmalloc but simplified. Specifically:
1. The small bins will exist as in dlmalloc
2. Large chunks will be part of a giant singular free list
3. The last split chunk will be maintained as a "designated victim"
4. All chunks will be organically 8 byte aligned