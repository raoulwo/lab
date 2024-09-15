# Object files

## Relocatable object files

You can find the following items in relocatable
object files:

* Machine-level instructions for functions (code)
* Values of initialized global variables (data)
* *Symbol table* containing all symbols

Why are the intermediate object files called
*relocatable*? That's because when linking them
together to create a single executable the
machine-level instructions from one relocatable
object file must be put next to machine-level
instructions from another relocatable object file.
The instructions therefore have to be easily
*movable* or *relocatable*.

That's why instructions have **no addresses** in
relocatable objects. They only obtain addresses
after the linking process.

We can explore the contents of relocatable object
files using the following commands:

```sh
# First, we need to create the object files.
gcc -c main.c
gcc -c max.c
```

```sh
# Now, we can look at the `ELF` contents.
# `-h` outputs the ELF file header
# `-l` outputs the segments
# `-S` outputs the sections
readelf -hlS max.o
```

Notable sections are the following:

* `.text` (`.code`) contains machine-level instructions
* `.data` contains values of initialized global vars
* `.bss` contains number of bytes required for
  uninitialized global variables
* `.symtab` contains the symbol table

If you inspect the symbol table of the relocatable
object files, you can see the addresses assigned to
the machine-level instructions of the symbols `max`
and `max3`:

```sh
readelf -s max.o
```

These addresses aren't the final addresses that the
machine instructions are placed at, they are meant
to be *relocated*.

## Executable object files

Executable object files contain the same contents
as relocatable object files, they can be arranged
differently however.

ELF executable object files contain more *sections*
than relocatable object files. Additionally, they
also contain *segments*. Each segment consists of
zero or more sections. The sections are put into
segments based on their contents. For example,
all sections containing machine-level instructions
are part of the same segment.

Let's take a look at the contents of an executable:

```sh
gcc main.o max.o -o max
```

```sh
readelf -hlS max
```

(Take a look in the book for more information about
the output if you're lost)

When taking a look at the symbol table, you can see
that the symbols have their final addresses:

* In executable object files, addresses are absolute
* In shared object files, *relative addresses are
  absolute

```sh
readelf -s max
```

## Static libraries (archives)

Static libraries are just *Unix archives* made from
relocatable object files. Static libraries are usually
linked together with object files to form an executable.

NOTE: Static libraries **aren't** object files, they're
simply containers for object files.

In Unix-like systems static libraries are named with
prefix `lib` and have the suffix `.a` (for archive).
You can create a static library using this command:

```sh
ar crs libexample.a foo.o bar.o ... baz.o
```

## Dynamic libraries (shared objects)

Dynamic libraries aren't part of the final executable,
instead they are linked against the executable at runtime.
This makes it possible to share the same shared object
between different processes.

When linking against shared libraries, all dependencies
are resolved at *linking time*, this isn't the case with
dynamic libraries. It's possible that some undefined symbols
aren't yet resolved at linking time. Linking dynamic
libraries at runtime requires a different process than
linking against static libraries. This process is performed
by *dynamic linkers* or *loaders*.

When loading a process that's about to be launched, a
shared object file will be *loaded* and mapped to a memory
region accessible by the process.

NOTE: Both executable and shared object files contain
segments in their ELF structure which are themselves made
up of zero or more sections. The difference between
executables and shared objects is that executables have
absolute addresses, shared objects have *relative absolute
addresses* making it possible to link them against various
processes. Another difference is that some segments required
for loading an executable file aren't present in shared
objects.

The term *relative absolute addresses* is kind of confusing.
It just means that the addresses in shared objects have fixed
offsets, this means their distance in memory is already
determined.
