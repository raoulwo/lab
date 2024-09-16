# Artifacts

Building C/C++ projects can lead to the following
*build artifacts* or *build products*:

1. Executable files:
    * `.out` in Unix-like systems
    * `.exe` in Windows
2. Static libraries:
    * `.a` in Unix-like systems
    * `.lib` in Windows
3. Dynamic libraries:
    * `.so` in Unix-like systems
    * `.dylib` in MacOS
    * `.dll` in Windows

All three of these possible build artifacts are
**object files**, note that *relocatable* object
files don't count as build artifacts since they
are just temporary products.

## Executable object files

Executable binaries can be run as *processes*.
They must contain an entry point where machine-level
instructions are executed. While the entry point
for C code is the `main` function, the entry points
for executable object files are *platform-dependent*.

## Static libraries (archives)

Static libraries are just archive files containing
multiple relocatable object files. This means static
libraries aren't produced by the linker directly,
instead they are created by the *default archive
program* of the system. In Unix-like systems it's the
`ar` program.

Since static libraries are just archives of intermediate
object files, they're usually linked with other
relocatable object files to produce executables. These
static libraries then become part of the executable at
*link time* (in contrast to dynamic libraries).

## Dynamic libraries (shared objects)

Shared objects aren't created by archiving tools,
they are directly created by the linker. Unlike
static libraries which become part of the final
executable at *link time*, dynamic libraries are
loaded into running processes at *runtime*. They
can even be shared by multiple processes at the same
time, hence the name *shared object*.
