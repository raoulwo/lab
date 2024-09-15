# Application Binary Interface (ABI)

ABIs guarantees the compatibility of two programs
at the level of their machine-level instructions,
together with their corresponding object files.
It's for example not possible for a program to use
a dynamic/static library that has a different ABI.
Executable binaries also can't be executed on
systems with different ABIs that the binary was
originally built for.

ABIs cover the following things:

* Instruction sets of the target architecture:
  * Processor instructions
  * Memory layout
  * Endianness
  * Registers
  * ...
* Data types, their sizes and alignment policies
* Function calling conventions:
  * Stack frame structure
  * Pushing order of elements
* How system calls should be invoked
* Used object file format
* Name mangling, vtable layout (C++)

The System V ABI is the most widely used ABI
standard for Unix-like systems like Linux and BSD.
The *Executable and Linking Format (ELF)* is the
standard object file format used in the System V
ABI.
